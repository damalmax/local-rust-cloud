use crate::tests::fixture::tag;

#[tokio::test]
async fn tag_server_certificate() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .upload_server_certificate()
        .server_certificate_name("TestCertificate1")
        .path("/servers/")
        .certificate_body(include_str!("resources/cert.pem"))
        .private_key(include_str!("resources/key.pem"))
        .tags(tag("key-1", "value-1"))
        .tags(tag("key-2", "value-2"))
        .tags(tag("key-3", "value-3"))
        .tags(tag("key-4", "value-4"))
        .tags(tag("key-5", "value-5"))
        .send()
        .await
        .expect("Failed to upload server certificate");

    client
        .tag_server_certificate()
        .server_certificate_name("TestCertificate1")
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .tags(tag("key3", "value3"))
        .tags(tag("key4", "value4"))
        .tags(tag("key5", "value5"))
        .send()
        .await
        .expect("Failed to tag IAM role");

    ctx.stop_server().await;
}

#[tokio::test]
async fn tag_server_certificate_limit_exceeded() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .upload_server_certificate()
        .server_certificate_name("TestCertificate1")
        .path("/servers/")
        .certificate_body(include_str!("resources/cert.pem"))
        .private_key(include_str!("resources/key.pem"))
        .tags(tag("key-1", "value-1"))
        .tags(tag("key-2", "value-2"))
        .tags(tag("key-3", "value-3"))
        .tags(tag("key-4", "value-4"))
        .tags(tag("key-5", "value-5"))
        .send()
        .await
        .expect("Failed to upload server certificate");

    let tags = (5..=51)
        .map(|i| tag(format!("key-{}", i).as_str(), format!("value-{}", i).as_str()))
        .collect();
    let result = client
        .tag_server_certificate()
        .server_certificate_name("TestCertificate1")
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
    assert_eq!(error.meta().message().unwrap(), "Cannot assign more than 50 tags to IAM server certificate.");

    ctx.stop_server().await;
}

#[tokio::test]
async fn test_server_certificate_with_replacement() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .upload_server_certificate()
        .server_certificate_name("TestCertificate1")
        .path("/servers/")
        .certificate_body(include_str!("resources/cert.pem"))
        .private_key(include_str!("resources/key.pem"))
        .tags(tag("key-1", "value-1"))
        .tags(tag("key-2", "value-2"))
        .tags(tag("key-3", "value-3"))
        .tags(tag("key-4", "value-4"))
        .tags(tag("key-5", "value-5"))
        .send()
        .await
        .expect("Failed to upload server certificate");

    let tags = (1..=50)
        .map(|i| tag(format!("key-{}", i).as_str(), format!("value-{}", i).as_str()))
        .collect();
    client
        .tag_server_certificate()
        .server_certificate_name("TestCertificate1")
        .set_tags(Some(tags))
        .send()
        .await
        .expect("Failed to assign maximum allowed number of tags with value replacements to IAM server certificate");

    ctx.stop_server().await;
}
