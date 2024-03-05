const USER_NAME: &str = "user1";

#[tokio::test]
async fn list_signing_certificates_empty() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .create_user()
        .user_name(USER_NAME)
        .path("/")
        .send()
        .await
        .expect("Failed to create IAM user");

    let result = client
        .list_signing_certificates()
        .user_name(USER_NAME)
        .max_items(10)
        .send()
        .await
        .expect("Failed to get a list of signing certificates");

    assert!(result.certificates().is_empty());
    ctx.stop_server().await;
}
