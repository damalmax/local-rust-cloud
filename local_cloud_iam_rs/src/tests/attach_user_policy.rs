use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;

#[tokio::test]
async fn attach_user_policy() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    super::fixture::create_user(&client, "user1", "/", None, None)
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

    client
        .attach_user_policy()
        .user_name("user1")
        .policy_arn(policy_output.policy().unwrap().arn().unwrap())
        .send()
        .await
        .expect("Failed to attach role policy");

    ctx.stop_server().await;
}
