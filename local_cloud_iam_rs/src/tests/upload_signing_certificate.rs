use local_cloud_testing::assertions::assert_not_empty;

#[tokio::test]
async fn upload_signing_certificate() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    super::fixture::create_user(&client, "TestUser1", "/", None, None)
        .await
        .expect("Failed to create IAM user");

    let upload_signing_certificate_output = client
        .upload_signing_certificate()
        .certificate_body(include_str!("resources/cert.pem").trim())
        .user_name("TestUser1")
        .send()
        .await
        .expect("Failed to upload server certificate");

    assert!(upload_signing_certificate_output.certificate().is_some());
    let certificate = upload_signing_certificate_output.certificate().unwrap();
    assert_eq!(certificate.status().as_str(), "Active");
    assert_eq!(certificate.user_name(), "TestUser1");
    assert_eq!(certificate.certificate_body(), include_str!("resources/cert.pem").trim());
    assert!(certificate.upload_date().is_some());
    assert_not_empty(certificate.certificate_id());
}
