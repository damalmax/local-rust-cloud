#[tokio::test]
async fn list_server_certificates_empty() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let result = client
        .list_server_certificates()
        .max_items(10)
        .send()
        .await
        .expect("Failed to get a list of server users");

    assert!(result.server_certificate_metadata_list().is_empty());
}
