use crate::tests::fixture::{CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY, CREATE_USER_PERMISSIONS_BOUNDARY};

#[tokio::test]
async fn put_role_policy() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .create_role()
        .role_name("Test-Role")
        .path("/")
        .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
        .send()
        .await
        .expect("Failed to create IAM role");

    let result = client
        .put_role_policy()
        .role_name("Test-Role")
        .policy_name("test_policy_1")
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .send()
        .await;

    assert!(result.is_ok());
}
