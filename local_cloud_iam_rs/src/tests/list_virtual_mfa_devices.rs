use local_cloud_testing::assertions::assert_not_empty;

const USER_NAME: &str = "test-user-1";

#[actix_rt::test]
async fn list_virtual_mfa_devices_empty() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let result = client
        .list_virtual_mfa_devices()
        .max_items(10)
        .send()
        .await
        .expect("Failed to get a list of IAM virtual MFA devices");

    assert!(result.virtual_mfa_devices().is_empty());

    ctx.stop_server().await;
}

#[actix_rt::test]
async fn list_virtual_mfa_devices_with_attached_user() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    super::fixture::create_user(&client, USER_NAME, "/", None, None)
        .await
        .expect("Failed to create IAM user");

    let create_mfa_device_output = client
        .create_virtual_mfa_device()
        .virtual_mfa_device_name("MfaDeviceName")
        .send()
        .await
        .expect("Failed to create Virtual MFA device");

    client
        .enable_mfa_device()
        .serial_number(create_mfa_device_output.virtual_mfa_device().unwrap().serial_number())
        .user_name(USER_NAME)
        .authentication_code1("012349") // just random code
        .authentication_code2("234566") // just random code
        .send()
        .await
        .expect("Failed to enable MFA device for IAM user");

    let result = client
        .list_virtual_mfa_devices()
        .max_items(10)
        .send()
        .await
        .expect("Failed to get a list of IAM virtual MFA devices");

    assert_eq!(result.virtual_mfa_devices().len(), 1);
    assert!(result.marker().is_none());
    assert!(!result.is_truncated());

    let mfa_device = result.virtual_mfa_devices();
    assert!(mfa_device[0].enable_date().is_some());
    assert!(mfa_device[0].user().is_some());
    let user = mfa_device[0].user().unwrap();
    assert_not_empty(user.user_id());
    assert_not_empty(user.user_name());
    assert_not_empty(user.arn());
    assert_not_empty(user.path());

    ctx.stop_server().await;
}
