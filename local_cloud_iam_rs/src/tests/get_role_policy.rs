use crate::tests::fixture::{CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY, CREATE_USER_PERMISSIONS_BOUNDARY};
use local_cloud_testing::assertions::assert_not_empty;

#[tokio::test]
async fn get_role_policy() {
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

    client
        .put_role_policy()
        .role_name("Test-Role")
        .policy_name("test_policy_1")
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .send()
        .await
        .expect("Failed to put IAM role policy");

    let result = client
        .get_role_policy()
        .role_name("Test-Role")
        .policy_name("test_policy_1")
        .send()
        .await;

    assert!(result.is_ok());

    let output = result.unwrap();
    assert_eq!(output.role_name(), "Test-Role");
    assert_eq!(output.policy_name(), "test_policy_1");
    assert_not_empty(output.policy_document());
}
