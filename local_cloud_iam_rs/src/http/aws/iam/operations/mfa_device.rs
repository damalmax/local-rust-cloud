use std::io::Cursor;

use aws_sdk_iam::operation::create_virtual_mfa_device::CreateVirtualMfaDeviceOutput;
use aws_sdk_iam::types::VirtualMfaDevice;
use aws_smithy_types::Blob;
use base64::prelude::BASE64_URL_SAFE;
use base64::Engine;
use chrono::Utc;
use image::{ImageOutputFormat, Luma};
use qrcode::QrCode;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::mfa_device::InsertMfaDevice;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::create_virtual_mfa_device_request::CreateVirtualMfaDeviceRequest;
use crate::http::aws::iam::{constants, db};

pub(crate) async fn create_virtual_mfa_device(
    ctx: &OperationCtx, input: &CreateVirtualMfaDeviceRequest, db: &LocalDb,
) -> Result<CreateVirtualMfaDeviceOutput, OperationError> {
    input.validate("$")?;

    // TODO: add check to allow to register up to 8 MFA devices per user (it is possible to register up to 8 devices to ROOT user also)
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

    let mut device_tags = super::tag::prepare_for_insert(input.tags(), insert_mfa_device.id.unwrap());
    db::mfa_device_tags::save_all(&mut tx, &mut device_tags).await?;

    // Using account ID since User is not available when we register a new MFA device.
    let account_name = format!("{:0>12}", ctx.account_id);
    let secret = BASE64_URL_SAFE.encode(&insert_mfa_device.seed);
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
