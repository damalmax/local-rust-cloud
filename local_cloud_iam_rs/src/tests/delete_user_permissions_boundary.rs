use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;

#[tokio::test]
async fn delete_user_permissions_boundary() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let policy_output = super::fixture::create_policy(
        &client,
        "test-policy",
        "user-test-policy",
        "/division_abc/subdivision_xyz/",
        CREATE_USER_PERMISSIONS_BOUNDARY,
        None,
    )
    .await
    .unwrap();

    client
        .create_user()
        .user_name("user1")
        .permissions_boundary(policy_output.policy().unwrap().arn().unwrap())
        .send()
        .await
        .expect("Failed to create IAM user");

    client
        .delete_user_permissions_boundary()
        .user_name("user1")
        .send()
        .await
        .expect("Failed to delete permissions boundary from IAM user");

    ctx.stop_server().await;
}
