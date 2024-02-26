const DEVICE_NAME: &str = "TestMFADevice";
const USER_NAME: &str = "user1";

#[tokio::test]
async fn get_mfa_device() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_mfa_device_output = client
        .create_virtual_mfa_device()
        .virtual_mfa_device_name("mfa-device-1")
        .send()
        .await
        .expect("Failed to create Virtual MFA device");

    let serial_number = create_mfa_device_output.virtual_mfa_device().unwrap().serial_number();
    let result = client
        .get_mfa_device()
        .serial_number(serial_number)
        .send()
        .await
        .expect("Failed to get IAM MFA device");

    assert!(result.enable_date().is_none()); // MFA device is not enabled
    assert_eq!(result.serial_number(), serial_number);
    assert!(result.user_name().is_none()); // MFA device is not attached to any user
    assert!(result.certifications().is_none()); // Virtual MFA device doesn't have certifications

    ctx.stop_server().await;
}

#[tokio::test]
async fn get_mfa_device_with_attached_user() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    super::fixture::create_user(&client, USER_NAME, "/", None, None)
        .await
        .expect("Failed to create IAM user");

    let create_mfa_device_output = client
        .create_virtual_mfa_device()
        .virtual_mfa_device_name(DEVICE_NAME)
        .send()
        .await
        .expect("Failed to create Virtual MFA device");

    let serial_number = create_mfa_device_output.virtual_mfa_device().unwrap().serial_number();

    client
        .enable_mfa_device()
        .serial_number(serial_number)
        .user_name(USER_NAME)
        .authentication_code1("012345") // just random code
        .authentication_code2("234566") // just random code
        .send()
        .await
        .expect("Failed to enable MFA device for IAM user");

    let result = client
        .get_mfa_device()
        .serial_number(serial_number)
        .send()
        .await
        .expect("Failed to get IAM MFA device");

    assert!(result.user_name().is_some());
    assert_eq!(result.user_name().unwrap(), USER_NAME);

    ctx.stop_server().await;
}
