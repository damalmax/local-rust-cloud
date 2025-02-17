const USER_NAME: &str = "test-user-1";
const NEW_USER_NAME: &str = "test-user-2";

#[tokio::test]
async fn update_user() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .create_user()
        .user_name(USER_NAME)
        .path("/")
        .send()
        .await
        .expect("Failed to create IAM user");

    client
        .update_user()
        .user_name(USER_NAME)
        .new_user_name(NEW_USER_NAME)
        .new_path("/group/")
        .send()
        .await
        .expect("Failed to update user");
}
