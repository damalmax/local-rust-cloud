use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;

const USER_NAME: &str = "test_user";
const POLICY_NAME: &str = "test_policy_1";

#[tokio::test]
async fn delete_user_policy() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    super::fixture::create_user(&client, USER_NAME, "/", None, None)
        .await
        .expect("Failed to create IAM user");

    client
        .put_user_policy()
        .user_name(USER_NAME)
        .policy_name(POLICY_NAME)
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .send()
        .await
        .expect("Failed to put user policy");

    client
        .delete_user_policy()
        .user_name(USER_NAME)
        .policy_name(POLICY_NAME)
        .send()
        .await
        .expect("Failed to delete user policy");

    ctx.stop_server().await;
}
