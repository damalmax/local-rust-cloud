#[tokio::test]
async fn list_users_empty() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let result = client
        .list_users()
        .max_items(10)
        .send()
        .await
        .expect("Failed to get a list of IAM users");

    assert!(result.users().is_empty());
}
