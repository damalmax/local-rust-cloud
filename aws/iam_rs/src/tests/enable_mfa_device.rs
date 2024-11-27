const DEVICE_NAME: &str = "TestMFADevice";
const USER_NAME: &str = "user1";

#[tokio::test]
async fn enable_mfa_device() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
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

    client
        .enable_mfa_device()
        .serial_number(create_mfa_device_output.virtual_mfa_device().unwrap().serial_number())
        .user_name(USER_NAME)
        .authentication_code1("012345") // just random code
        .authentication_code2("234566") // just random code
        .send()
        .await
        .expect("Failed to enable MFA device for IAM user");
}
