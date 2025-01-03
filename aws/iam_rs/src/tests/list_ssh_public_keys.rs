const USER_NAME: &str = "user1";

#[tokio::test]
async fn list_ssh_public_keys() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
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
        .list_ssh_public_keys()
        .user_name(USER_NAME)
        .max_items(10)
        .send()
        .await
        .expect("Failed to get a list of SSH public keys");

    assert!(result.ssh_public_keys().is_empty());
}
