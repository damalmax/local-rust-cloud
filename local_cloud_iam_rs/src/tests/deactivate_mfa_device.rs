const DEVICE_NAME: &str = "TestMFADevice";
const USER_NAME: &str = "user1";

#[tokio::test]
async fn deactivate_mfa_device() {
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

    client
        .deactivate_mfa_device()
        .user_name(USER_NAME)
        .serial_number(serial_number)
        .send()
        .await
        .expect("Failed to deactivate MFA device for IAM user");
}

#[tokio::test]
async fn deactivate_mfa_device_user_not_found() {
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
        .deactivate_mfa_device()
        .user_name("user2")
        .serial_number(serial_number)
        .send()
        .await;

    assert!(result.is_err());
    let sdk_error = result.unwrap_err();
    assert_eq!(404u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_no_such_entity_exception());
    assert_eq!("NoSuchEntity", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(error.meta().message().unwrap(), "IAM user with name 'user2' doesn't exist.");
}

#[tokio::test]
async fn deactivate_mfa_device_not_found() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    super::fixture::create_user(&client, USER_NAME, "/", None, None)
        .await
        .expect("Failed to create IAM user");

    let serial_number = "arn:aws:iam::000000000001:mfa/ExampleName";

    let result = client
        .deactivate_mfa_device()
        .user_name(USER_NAME)
        .serial_number(serial_number)
        .send()
        .await;

    assert!(result.is_err());
    let sdk_error = result.unwrap_err();
    assert_eq!(404u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_no_such_entity_exception());
    assert_eq!("NoSuchEntity", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(
        error.meta().message().unwrap(),
        "IAM MFA device with serial number 'arn:aws:iam::000000000001:mfa/ExampleName' doesn't exist."
    );
}

#[tokio::test]
async fn deactivate_mfa_device_assigned_to_different_user() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    super::fixture::create_user(&client, USER_NAME, "/", None, None)
        .await
        .expect("Failed to create IAM user");

    super::fixture::create_user(&client, "user2", "/", None, None)
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
        .deactivate_mfa_device()
        .user_name("user2")
        .serial_number(serial_number)
        .send()
        .await;

    assert!(result.is_err());
    let sdk_error = result.unwrap_err();
    assert_eq!(404u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_no_such_entity_exception());
    assert_eq!("NoSuchEntity", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(error.meta().message().unwrap(), "Failed to disable MFA device. It is assigned to a different user.");
}
