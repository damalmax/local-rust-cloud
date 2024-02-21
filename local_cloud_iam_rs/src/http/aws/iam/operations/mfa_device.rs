use std::io::Cursor;

use aws_sdk_iam::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceOutput;
use aws_sdk_iam::operation::enable_mfa_device::EnableMfaDeviceOutput;
use aws_sdk_iam::operation::get_mfa_device::GetMfaDeviceOutput;
use aws_sdk_iam::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesOutput;
use aws_sdk_iam::operation::tag_mfa_device::TagMfaDeviceOutput;
use aws_sdk_iam::types::VirtualMfaDevice;
use aws_smithy_types::{Blob, DateTime};
use chrono::Utc;
use data_encoding::BASE64;
use image::{ImageOutputFormat, Luma};
use qrcode::QrCode;
use sqlx::{Executor, Sqlite};

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::mfa_device::{
    EnableMfaDeviceQuery, InsertMfaDevice, ListVirtualMfaDevicesQuery, SelectMfaDevice,
};
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::create_virtual_mfa_device_request::CreateVirtualMfaDeviceRequest;
use crate::http::aws::iam::types::enable_mfa_device_request::EnableMfaDeviceRequest;
use crate::http::aws::iam::types::get_mfa_device_request::GetMfaDeviceRequest;
use crate::http::aws::iam::types::list_virtual_mfa_devices_request::ListVirtualMfaDevicesRequest;
use crate::http::aws::iam::types::tag_mfa_device_request::TagMfaDeviceRequest;
use crate::http::aws::iam::{constants, db};

pub(crate) async fn create_virtual_mfa_device(
    ctx: &OperationCtx, input: &CreateVirtualMfaDeviceRequest, db: &LocalDb,
) -> Result<CreateVirtualMfaDeviceOutput, OperationError> {
    input.validate("$")?;

    let current_time = Utc::now().timestamp();

    let mut tx = db.new_tx().await?;

    let path = input.path().unwrap_or("/").to_owned();
    let device_name = input.virtual_mfa_device_name().unwrap().trim();
    let arn = format!("arn:aws:iam::{:0>12}:mfa{}{}", ctx.account_id, &path, device_name);
    let seed = local_cloud_common::random::generate_bytes_slice(constants::mfa::SEED_LENGTH);

    let mut insert_mfa_device = InsertMfaDevice {
        id: None,
        account_id: ctx.account_id,
        serial_number: arn.to_owned(),
        path,
        name: input.virtual_mfa_device_name().unwrap().to_owned(),
        seed,
        create_date: current_time,
    };

    db::mfa_device::create(&mut tx, &mut insert_mfa_device).await?;

    let devices_count = db::mfa_device::count(tx.as_mut(), ctx.account_id, None).await?;
    if devices_count > constants::mfa::DEVICE_MAX_COUNT_PER_USER {
        return Err(OperationError::new(
            ApiErrorKind::LimitExceeded,
            format!(
                "It is allowed to register up to {} MFA devices per user.",
                constants::mfa::DEVICE_MAX_COUNT_PER_USER
            )
            .as_str(),
        ));
    }

    let mut device_tags = super::tag::prepare_for_insert(input.tags(), insert_mfa_device.id.unwrap());
    db::Tags::MfaDevice.save_all(&mut tx, &mut device_tags).await?;

    // Using account ID since User is not available when we register a new MFA device.
    let account_name = format!("{:0>12}", ctx.account_id);
    let secret = BASE64.encode(&insert_mfa_device.seed);
    let qr_code_str = format!("otpauth://totp/{device_name}@{account_name}?secret={secret}");

    let code = QrCode::new(&qr_code_str).unwrap();
    let image = code.render::<Luma<u8>>().build();
    let mut image_bytes_cursor = Cursor::new(Vec::new());

    image
        .write_to(&mut image_bytes_cursor, ImageOutputFormat::Png)
        .map_err(|_| {
            OperationError::new(
                ApiErrorKind::ServiceFailure,
                "Failed to generate QRCode image. Please contact service team for resolution.",
            )
        })?;

    let device = VirtualMfaDevice::builder()
        .serial_number(&arn)
        .base32_string_seed(Blob::new(insert_mfa_device.seed.as_slice()))
        .qr_code_png(Blob::new(image_bytes_cursor.into_inner()))
        .set_tags(super::tag::prepare_for_output(&device_tags))
        .build()
        .unwrap();

    let output = CreateVirtualMfaDeviceOutput::builder()
        .virtual_mfa_device(device)
        .build();

    tx.commit().await?;

    Ok(output)
}

pub(crate) async fn find_id_by_serial_number<'a, E>(
    executor: E, account_id: i64, serial_number: &str,
) -> Result<i64, OperationError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::mfa_device::find_id_by_serial_number(executor, account_id, serial_number).await? {
        Some(mfa_device_id) => Ok(mfa_device_id),
        None => {
            return Err(OperationError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM MFA device with serial number '{}' doesn't exist.", serial_number).as_str(),
            ));
        }
    }
}

pub(crate) async fn find_by_serial_number<'a, E>(
    executor: E, account_id: i64, serial_number: &str, user_name: Option<&str>,
) -> Result<SelectMfaDevice, OperationError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::mfa_device::find_by_serial_number(executor, account_id, serial_number).await? {
        Some(mfa_device) => {
            if user_name.is_none()
                || (user_name.is_some()
                    && mfa_device.user_name.is_some()
                    && user_name.unwrap() == mfa_device.user_name.as_ref().unwrap())
            {
                Ok(mfa_device)
            } else {
                Err(OperationError::new(
                    ApiErrorKind::NoSuchEntity,
                    format!("IAM MFA device with serial number '{}' assigned to a different user.", serial_number)
                        .as_str(),
                ))
            }
        }
        None => {
            return Err(OperationError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM MFA device with serial number '{}' doesn't exist.", serial_number).as_str(),
            ));
        }
    }
}

pub(crate) async fn get_mfa_device(
    ctx: &OperationCtx, input: &GetMfaDeviceRequest, db: &LocalDb,
) -> Result<GetMfaDeviceOutput, OperationError> {
    input.validate("$")?;

    let mut connection = db.new_connection().await?;

    let serial_number = input.serial_number().unwrap();
    let user_name = input.user_name();
    let select_mfa_device =
        find_by_serial_number(connection.as_mut(), ctx.account_id, serial_number, user_name).await?;

    let output = GetMfaDeviceOutput::builder()
        .serial_number(&select_mfa_device.serial_number)
        .set_enable_date(select_mfa_device.enable_date.map(|date| DateTime::from_secs(date)))
        .set_user_name(select_mfa_device.user_name)
        .set_certifications(None)
        .build()
        .unwrap();
    Ok(output)
}

pub(crate) async fn enable_mfa_device(
    ctx: &OperationCtx, input: &EnableMfaDeviceRequest, db: &LocalDb,
) -> Result<EnableMfaDeviceOutput, OperationError> {
    input.validate("$")?;

    let current_time = Utc::now().timestamp();

    let mut tx = db.new_tx().await?;

    let user_id = super::user::find_id_by_name(tx.as_mut(), ctx.account_id, input.user_name().unwrap()).await?;

    let mfa_device = find_by_serial_number(tx.as_mut(), ctx.account_id, input.serial_number().unwrap(), None).await?;
    if mfa_device.user_id.is_some() && mfa_device.enable_date.is_some() {
        return Err(OperationError::new(ApiErrorKind::EntityAlreadyExists, "MFA device is already activated."));
    }

    let query = EnableMfaDeviceQuery {
        id: mfa_device.id,
        enable_date: current_time,
        user_id,
        code1: input.authentication_code_1().unwrap().to_owned(),
        code2: input.authentication_code_2().unwrap().to_owned(),
    };

    db::mfa_device::enable(&mut tx, &query).await?;

    let output = EnableMfaDeviceOutput::builder().build();

    tx.commit().await?;
    Ok(output)
}

pub(crate) async fn list_virtual_mfa_devices(
    ctx: &OperationCtx, input: &ListVirtualMfaDevicesRequest, db: &LocalDb,
) -> Result<ListVirtualMfaDevicesOutput, OperationError> {
    input.validate("$")?;

    let mut connection = db.new_connection().await?;

    let query: ListVirtualMfaDevicesQuery = input.into();
    let found_mfa_devices = db::mfa_device::list_virtual(connection.as_mut(), ctx.account_id, &query).await?;

    let marker = super::common::create_encoded_marker(&query, found_mfa_devices.len())?;
    let mfa_devices = super::common::convert_and_limit(&found_mfa_devices, query.limit).unwrap_or_default();

    let output = ListVirtualMfaDevicesOutput::builder()
        .set_virtual_mfa_devices(Some(mfa_devices))
        .set_is_truncated(marker.as_ref().map(|_| true))
        .set_marker(marker)
        .build()
        .unwrap();
    Ok(output)
}

pub(crate) async fn tag_mfa_device(
    ctx: &OperationCtx, input: &TagMfaDeviceRequest, db: &LocalDb,
) -> Result<TagMfaDeviceOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let mfa_device_id =
        find_id_by_serial_number(tx.as_mut(), ctx.account_id, input.serial_number().unwrap().trim()).await?;
    let mut mfa_device_tags = super::tag::prepare_for_insert(input.tags(), mfa_device_id);

    db::Tags::MfaDevice.save_all(&mut tx, &mut mfa_device_tags).await?;
    let count = db::Tags::MfaDevice.count(tx.as_mut(), mfa_device_id).await?;
    if count > constants::tag::MAX_COUNT {
        return Err(OperationError::new(
            ApiErrorKind::LimitExceeded,
            format!("Cannot assign more than {} tags to IAM MFA device.", constants::tag::MAX_COUNT).as_str(),
        ));
    }

    let output = TagMfaDeviceOutput::builder().build();

    tx.commit().await?;

    Ok(output)
}
