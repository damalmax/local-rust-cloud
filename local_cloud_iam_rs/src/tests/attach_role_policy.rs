use crate::tests::fixture::{CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY, CREATE_USER_PERMISSIONS_BOUNDARY};

#[tokio::test]
async fn attach_role_policy() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let _create_role_output = client
        .create_role()
        .role_name("Test-Role")
        .path("/")
        .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
        .send()
        .await
        .expect("Failed to create IAM role");

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

    let _response = client
        .attach_role_policy()
        .role_name("Test-Role")
        .policy_arn(policy_output.policy().unwrap().arn().unwrap())
        .send()
        .await
        .expect("Failed to attach role policy");

    ctx.stop_server().await;
}
