use crate::tests::fixture::tag;

#[tokio::test]
async fn tag_mfa_device() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_virtual_mfa_device_output = client
        .create_virtual_mfa_device()
        .virtual_mfa_device_name("MFA-device-1")
        .tags(tag("key-1", "value-1"))
        .tags(tag("key-2", "value-2"))
        .tags(tag("key-3", "value-3"))
        .tags(tag("key-4", "value-4"))
        .tags(tag("key-5", "value-5"))
        .send()
        .await
        .expect("Failed to create Virtual MFA device");

    let serial_number = create_virtual_mfa_device_output
        .virtual_mfa_device()
        .unwrap()
        .serial_number();

    client
        .tag_mfa_device()
        .serial_number(serial_number)
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .tags(tag("key3", "value3"))
        .tags(tag("key4", "value4"))
        .tags(tag("key5", "value5"))
        .send()
        .await
        .expect("Failed to tag IAM MFA device");
}

#[tokio::test]
async fn tag_mfa_device_limit_exceeded() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_virtual_mfa_device_output = client
        .create_virtual_mfa_device()
        .virtual_mfa_device_name("MFA-device-1")
        .tags(tag("key-1", "value-1"))
        .tags(tag("key-2", "value-2"))
        .tags(tag("key-3", "value-3"))
        .tags(tag("key-4", "value-4"))
        .tags(tag("key-5", "value-5"))
        .send()
        .await
        .expect("Failed to create Virtual MFA device");

    let serial_number = create_virtual_mfa_device_output
        .virtual_mfa_device()
        .unwrap()
        .serial_number();

    let tags = (5..=51)
        .map(|i| tag(format!("key-{}", i).as_str(), format!("value-{}", i).as_str()))
        .collect();
    let result = client
        .tag_mfa_device()
        .serial_number(serial_number)
        .set_tags(Some(tags))
        .send()
        .await;

    assert!(result.is_err());
    let sdk_error = result.unwrap_err();
    assert_eq!(409u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_limit_exceeded_exception());
    assert_eq!("LimitExceeded", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(error.meta().message().unwrap(), "Cannot assign more than 50 tags to IAM MFA device.");
}

#[tokio::test]
async fn tag_mfa_device_with_replacement() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_virtual_mfa_device_output = client
        .create_virtual_mfa_device()
        .virtual_mfa_device_name("MFA-device-1")
        .tags(tag("key-1", "value-1"))
        .tags(tag("key-2", "value-2"))
        .tags(tag("key-3", "value-3"))
        .tags(tag("key-4", "value-4"))
        .tags(tag("key-5", "value-5"))
        .send()
        .await
        .expect("Failed to create Virtual MFA device");

    let serial_number = create_virtual_mfa_device_output
        .virtual_mfa_device()
        .unwrap()
        .serial_number();

    let tags = (1..=50)
        .map(|i| tag(format!("key-{}", i).as_str(), format!("value-{}", i).as_str()))
        .collect();
    client
        .tag_mfa_device()
        .serial_number(serial_number)
        .set_tags(Some(tags))
        .send()
        .await
        .expect("Failed to assign maximum allowed number of tags with value replacements to IAM MFA device");
}
