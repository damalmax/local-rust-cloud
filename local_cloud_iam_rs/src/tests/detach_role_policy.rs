use crate::tests::fixture::{CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY, CREATE_USER_PERMISSIONS_BOUNDARY};

const ROLE_NAME: &str = "Test-Role";

#[tokio::test]
async fn detach_role_policy() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let _create_role_output = client
        .create_role()
        .role_name(ROLE_NAME)
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

    let policy_arn = policy_output.policy().unwrap().arn().unwrap();

    client
        .attach_role_policy()
        .role_name(ROLE_NAME)
        .policy_arn(policy_arn)
        .send()
        .await
        .expect("Failed to attach role policy");

    client
        .detach_role_policy()
        .role_name(ROLE_NAME)
        .policy_arn(policy_arn)
        .send()
        .await
        .expect("Failed to detach policy from role");

    ctx.stop_server().await;
}
