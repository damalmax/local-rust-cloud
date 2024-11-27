use std::io::Cursor;

use aws_sdk_iam::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceOutput;
use aws_sdk_iam::operation::deactivate_mfa_device::DeactivateMfaDeviceOutput;
use aws_sdk_iam::operation::delete_virtual_mfa_device::DeleteVirtualMfaDeviceOutput;
use aws_sdk_iam::operation::enable_mfa_device::EnableMfaDeviceOutput;
use aws_sdk_iam::operation::get_mfa_device::GetMfaDeviceOutput;
use aws_sdk_iam::operation::list_mfa_device_tags::ListMfaDeviceTagsOutput;
use aws_sdk_iam::operation::list_mfa_devices::ListMfaDevicesOutput;
use aws_sdk_iam::operation::list_virtual_mfa_devices::ListVirtualMfaDevicesOutput;
use aws_sdk_iam::operation::resync_mfa_device::ResyncMfaDeviceOutput;
use aws_sdk_iam::operation::tag_mfa_device::TagMfaDeviceOutput;
use aws_sdk_iam::operation::untag_mfa_device::UntagMfaDeviceOutput;
use aws_sdk_iam::types::VirtualMfaDevice;
use aws_smithy_types::{Blob, DateTime};
use chrono::Utc;
use data_encoding::BASE64;
use image::{ImageOutputFormat, Luma};
use qrcode::QrCode;
use sqlx::{Executor, Sqlite, Transaction};

use validators::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::mfa_device::{
    EnableMfaDeviceQuery, InsertMfaDevice, ListVirtualMfaDevicesQuery, SelectMfaDevice,
};
use crate::http::aws::iam::db::types::tags::ListTagsQuery;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::create_virtual_mfa_device::CreateVirtualMfaDeviceRequest;
use crate::http::aws::iam::types::deactivate_mfa_device::DeactivateMfaDeviceRequest;
use crate::http::aws::iam::types::delete_virtual_mfa_device::DeleteVirtualMfaDeviceRequest;
use crate::http::aws::iam::types::enable_mfa_device::EnableMfaDeviceRequest;
use crate::http::aws::iam::types::get_mfa_device::GetMfaDeviceRequest;
use crate::http::aws::iam::types::list_mfa_device_tags::ListMfaDeviceTagsRequest;
use crate::http::aws::iam::types::list_mfa_devices::ListMfaDevicesRequest;
use crate::http::aws::iam::types::list_virtual_mfa_devices::ListVirtualMfaDevicesRequest;
use crate::http::aws::iam::types::resync_mfa_device::ResyncMfaDeviceRequest;
use crate::http::aws::iam::types::tag_mfa_device::TagMfaDeviceRequest;
use crate::http::aws::iam::types::untag_mfa_device::UntagMfaDeviceRequest;
use crate::http::aws::iam::{constants, db};

pub(crate) async fn create_virtual_mfa_device<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &CreateVirtualMfaDeviceRequest,
) -> Result<CreateVirtualMfaDeviceOutput, ActionError> {
    input.validate("$")?;

    let current_time = Utc::now().timestamp();

    let path = input.path().unwrap_or("/").to_owned();
    let device_name = input.virtual_mfa_device_name().unwrap().trim();
    let arn = format!("arn:aws:iam::{:0>12}:mfa{}{}", ctx.account_id, &path, device_name);
    let seed = utils::random::generate_bytes_slice(constants::mfa::SEED_LENGTH);

    let mut insert_mfa_device = InsertMfaDevice {
        id: None,
        account_id: ctx.account_id,
        serial_number: arn.to_owned(),
        path,
        name: input.virtual_mfa_device_name().unwrap().to_owned(),
        seed,
        create_date: current_time,
    };

    db::mfa_device::create(tx, &mut insert_mfa_device).await?;

    let devices_count = db::mfa_device::count(tx.as_mut(), ctx.account_id, None).await?;
    if devices_count > constants::mfa::DEVICE_MAX_COUNT_PER_USER {
        return Err(ActionError::new(
            ApiErrorKind::LimitExceeded,
            format!(
                "It is allowed to register up to {} MFA devices per user.",
                constants::mfa::DEVICE_MAX_COUNT_PER_USER
            )
            .as_str(),
        ));
    }

    let mut device_tags = super::tag::prepare_for_db(input.tags(), insert_mfa_device.id.unwrap());
    db::Tags::MfaDevice.save_all(tx, &mut device_tags).await?;

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
            ActionError::new(
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
    Ok(output)
}

pub(crate) async fn find_id_by_serial_number<'a, E>(
    executor: E, account_id: i64, serial_number: &str,
) -> Result<i64, ActionError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::mfa_device::find_id_by_serial_number(executor, account_id, serial_number).await? {
        Some(mfa_device_id) => Ok(mfa_device_id),
        None => {
            return Err(ActionError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM MFA device with serial number '{}' doesn't exist.", serial_number).as_str(),
            ));
        }
    }
}

pub(crate) async fn find_by_serial_number<'a, E>(
    executor: E, account_id: i64, serial_number: &str, user_name: Option<&str>,
) -> Result<SelectMfaDevice, ActionError>
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
                Err(ActionError::new(
                    ApiErrorKind::NoSuchEntity,
                    format!("IAM MFA device with serial number '{}' assigned to a different user.", serial_number)
                        .as_str(),
                ))
            }
        }
        None => {
            return Err(ActionError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM MFA device with serial number '{}' doesn't exist.", serial_number).as_str(),
            ));
        }
    }
}

pub(crate) async fn get_mfa_device<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetMfaDeviceRequest,
) -> Result<GetMfaDeviceOutput, ActionError> {
    input.validate("$")?;

    let serial_number = input.serial_number().unwrap();
    let user_name = input.user_name();
    let select_mfa_device = find_by_serial_number(tx.as_mut(), ctx.account_id, serial_number, user_name).await?;

    let output = GetMfaDeviceOutput::builder()
        .serial_number(&select_mfa_device.serial_number)
        .set_enable_date(select_mfa_device.enable_date.map(|date| DateTime::from_secs(date)))
        .set_user_name(select_mfa_device.user_name)
        .set_certifications(None)
        .build()
        .unwrap();
    Ok(output)
}

pub(crate) async fn enable_mfa_device<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &EnableMfaDeviceRequest,
) -> Result<EnableMfaDeviceOutput, ActionError> {
    input.validate("$")?;

    let current_time = Utc::now().timestamp();

    let user_id = super::user::find_id_by_name(tx.as_mut(), ctx.account_id, input.user_name().unwrap()).await?;

    let mfa_device = find_by_serial_number(tx.as_mut(), ctx.account_id, input.serial_number().unwrap(), None).await?;
    if mfa_device.user_id.is_some() && mfa_device.enable_date.is_some() {
        return Err(ActionError::new(ApiErrorKind::EntityAlreadyExists, "MFA device is already activated."));
    }

    let query = EnableMfaDeviceQuery {
        id: mfa_device.id,
        enable_date: current_time,
        user_id,
        code1: input.authentication_code_1().unwrap().to_owned(),
        code2: input.authentication_code_2().unwrap().to_owned(),
    };

    db::mfa_device::enable(tx, &query).await?;

    let output = EnableMfaDeviceOutput::builder().build();
    Ok(output)
}

pub(crate) async fn list_virtual_mfa_devices<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListVirtualMfaDevicesRequest,
) -> Result<ListVirtualMfaDevicesOutput, ActionError> {
    input.validate("$")?;

    let query: ListVirtualMfaDevicesQuery = input.into();
    let found_mfa_devices = db::mfa_device::list_virtual(tx.as_mut(), ctx.account_id, &query).await?;

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

pub(crate) async fn tag_mfa_device<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &TagMfaDeviceRequest,
) -> Result<TagMfaDeviceOutput, ActionError> {
    input.validate("$")?;

    let mfa_device_id =
        find_id_by_serial_number(tx.as_mut(), ctx.account_id, input.serial_number().unwrap().trim()).await?;
    let mut mfa_device_tags = super::tag::prepare_for_db(input.tags(), mfa_device_id);

    db::Tags::MfaDevice.save_all(tx, &mut mfa_device_tags).await?;
    let count = db::Tags::MfaDevice.count(tx.as_mut(), mfa_device_id).await?;
    if count > constants::tag::MAX_COUNT {
        return Err(ActionError::new(
            ApiErrorKind::LimitExceeded,
            format!("Cannot assign more than {} tags to IAM MFA device.", constants::tag::MAX_COUNT).as_str(),
        ));
    }

    let output = TagMfaDeviceOutput::builder().build();
    Ok(output)
}

pub(crate) async fn untag_mfa_device<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &UntagMfaDeviceRequest,
) -> Result<UntagMfaDeviceOutput, ActionError> {
    input.validate("$")?;

    let mfa_device_id =
        find_id_by_serial_number(tx.as_mut(), ctx.account_id, input.serial_number().unwrap().trim()).await?;

    db::Tags::MfaDevice
        .delete_all(tx, mfa_device_id, &input.tag_keys())
        .await?;

    let output = UntagMfaDeviceOutput::builder().build();
    Ok(output)
}

pub(crate) async fn list_mfa_device_tags<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListMfaDeviceTagsRequest,
) -> Result<ListMfaDeviceTagsOutput, ActionError> {
    input.validate("$")?;

    let mfa_device_id =
        find_id_by_serial_number(tx.as_mut(), ctx.account_id, input.serial_number().unwrap().trim()).await?;

    let query = ListTagsQuery::new(input.max_items(), input.marker_type());
    let found_tags = db::Tags::MfaDevice.list(tx.as_mut(), mfa_device_id, &query).await?;

    let tags = super::common::convert_and_limit(&found_tags, query.limit);
    let marker = super::common::create_encoded_marker(&query, found_tags.len())?;

    let output = ListMfaDeviceTagsOutput::builder()
        .set_tags(tags)
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build()
        .unwrap();
    Ok(output)
}

pub(crate) async fn deactivate_mfa_device<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &DeactivateMfaDeviceRequest,
) -> Result<DeactivateMfaDeviceOutput, ActionError> {
    input.validate("$")?;

    let user_id = super::user::find_id_by_name(tx.as_mut(), ctx.account_id, input.user_name().unwrap()).await?;

    let serial_number = input.serial_number().unwrap().trim();
    let mfa_device_id = find_id_by_serial_number(tx.as_mut(), ctx.account_id, serial_number).await?;

    let is_disabled = db::mfa_device::disable(tx, mfa_device_id, user_id).await?;
    if !is_disabled {
        return Err(ActionError::new(
            ApiErrorKind::NoSuchEntity,
            "Failed to disable MFA device. It is assigned to a different user.",
        ));
    }

    let output = DeactivateMfaDeviceOutput::builder().build();
    Ok(output)
}

pub(crate) async fn list_mfa_devices<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListMfaDevicesRequest,
) -> Result<ListMfaDevicesOutput, ActionError> {
    input.validate("$")?;

    let output = ListMfaDevicesOutput::builder().build().unwrap();

    Ok(output)
}

pub(crate) async fn resync_mfa_device<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ResyncMfaDeviceRequest,
) -> Result<ResyncMfaDeviceOutput, ActionError> {
    input.validate("$")?;

    let output = ResyncMfaDeviceOutput::builder().build();
    Ok(output)
}

pub(crate) async fn delete_virtual_mfa_device<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &DeleteVirtualMfaDeviceRequest,
) -> Result<DeleteVirtualMfaDeviceOutput, ActionError> {
    input.validate("$")?;

    let output = DeleteVirtualMfaDeviceOutput::builder().build();
    Ok(output)
}
