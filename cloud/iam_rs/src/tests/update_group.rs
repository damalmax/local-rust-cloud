const GROUP_NAME: &str = "test-group-1";
const NEW_GROUP_NAME: &str = "test-group-2";

#[tokio::test]
async fn update_group() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .create_group()
        .group_name(GROUP_NAME)
        .path("/")
        .send()
        .await
        .expect("Failed to create IAM group");

    client
        .update_group()
        .group_name(GROUP_NAME)
        .new_group_name(NEW_GROUP_NAME)
        .new_path("/group/")
        .send()
        .await
        .expect("Failed to update IAM group");
}
