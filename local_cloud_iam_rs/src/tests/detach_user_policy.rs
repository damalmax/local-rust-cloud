use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;

const USER_NAME: &str = "user1";

#[tokio::test]
async fn attach_user_policy() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    super::fixture::create_user(&client, USER_NAME, "/", None, None)
        .await
        .expect("Failed to create IAM user");

    let policy_output = super::fixture::create_policy(
        &client,
        "test-policy",
        "user-test-policy",
        "/division_abc/subdivision_xyz/",
        CREATE_USER_PERMISSIONS_BOUNDARY,
        None,
    )
    .await
    .expect("Failed to create IAM policy");

    let policy_arn = policy_output.policy().unwrap().arn().unwrap();
    client
        .attach_user_policy()
        .user_name(USER_NAME)
        .policy_arn(policy_arn)
        .send()
        .await
        .expect("Failed to attach user policy");

    client
        .detach_user_policy()
        .user_name(USER_NAME)
        .policy_arn(policy_arn)
        .send()
        .await
        .expect("Failed to detach user policy");

    ctx.stop_server().await;
}
