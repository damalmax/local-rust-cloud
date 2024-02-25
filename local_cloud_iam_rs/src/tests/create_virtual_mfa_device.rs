use image::ImageFormat;

use crate::tests::fixture::tag;

const DEVICE_NAME: &str = "TestDevice";

#[tokio::test]
async fn create_virtual_mfa_device() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let result = client
        .create_virtual_mfa_device()
        .virtual_mfa_device_name(DEVICE_NAME)
        .tags(tag("key-1", "value-1"))
        .tags(tag("key-2", "value-2"))
        .tags(tag("key-3", "value-3"))
        .tags(tag("key-4", "value-4"))
        .tags(tag("key-5", "value-5"))
        .send()
        .await
        .expect("Failed to create Virtual MFA device");

    let virtual_mfa_device = result.virtual_mfa_device().unwrap();
    assert_eq!(virtual_mfa_device.serial_number(), "arn:aws:iam::000000000001:mfa/TestDevice");
    assert_eq!(virtual_mfa_device.tags().len(), 5);
    assert!(virtual_mfa_device.base32_string_seed().is_some());
    assert!(virtual_mfa_device.qr_code_png.is_some());

    let qr_code_image_bytes = virtual_mfa_device.qr_code_png().unwrap().as_ref();
    let image = image::load_from_memory_with_format(&qr_code_image_bytes, ImageFormat::Png);
    assert!(image.is_ok());

    ctx.stop_server().await;
}

#[tokio::test]
async fn create_virtual_mfa_device_limit_exceeded() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    for i in 1..=8 {
        client
            .create_virtual_mfa_device()
            .virtual_mfa_device_name(format!("mfa-device-{i}"))
            .send()
            .await
            .expect("Failed to create Virtual MFA device");
    }
    let result = client
        .create_virtual_mfa_device()
        .virtual_mfa_device_name(format!("mfa-device-9"))
        .send()
        .await;

    assert!(result.is_err());
    let sdk_error = result.unwrap_err();
    assert_eq!(409u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_limit_exceeded_exception());
    assert_eq!("LimitExceeded", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(error.meta().message().unwrap(), "It is allowed to register up to 8 MFA devices per user.");

    ctx.stop_server().await;
}
