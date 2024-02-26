use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;
use local_cloud_testing::assertions::assert_not_empty;

#[tokio::test]
async fn get_user_policy() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    super::fixture::create_user(&client, "user1", "/", None, None)
        .await
        .expect("Failed to create IAM user");

    client
        .put_user_policy()
        .user_name("user1")
        .policy_name("test_policy_1")
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .send()
        .await
        .expect("Failed to put AM user policy");

    let result = client
        .get_user_policy()
        .user_name("user1")
        .policy_name("test_policy_1")
        .send()
        .await;

    assert!(result.is_ok());

    let output = result.unwrap();
    assert_eq!(output.user_name(), "user1");
    assert_eq!(output.policy_name(), "test_policy_1");
    assert_not_empty(output.policy_document());

    ctx.stop_server().await;
}
