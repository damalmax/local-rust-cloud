use crate::tests::fixture::tag;

#[tokio::test]
async fn upload_server_certificate() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let upload_server_certificate_output = client
        .upload_server_certificate()
        .server_certificate_name("TestCertificate1")
        .path("/servers/")
        .certificate_body(include_str!("resources/cert.pem"))
        .private_key(include_str!("resources/key.pem"))
        .tags(tag("key-1", "value-1"))
        .tags(tag("key-2", "value-2"))
        .send()
        .await
        .expect("Failed to upload server certificate");

    assert!(upload_server_certificate_output.server_certificate_metadata().is_some());
    assert_eq!(upload_server_certificate_output.tags().len(), 2);
    let metadata = upload_server_certificate_output.server_certificate_metadata().unwrap();
    assert_eq!(metadata.arn(), "arn:aws:iam::000000000001:server-certificate/servers/TestCertificate1");
    assert_eq!(metadata.path(), "/servers/");
    assert!(metadata.expiration.is_some());

    ctx.stop_server().await;
}
