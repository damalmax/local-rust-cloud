use crate::tests::fixture::{CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY, CREATE_USER_PERMISSIONS_BOUNDARY};

const ROLE_NAME: &str = "test-role-123";
const POLICY_NAME: &str = "test-policy-name";

#[tokio::test]
async fn delete_role_policy() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .create_role()
        .role_name(ROLE_NAME)
        .path("/")
        .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
        .send()
        .await
        .expect("Failed to create IAM role");

    client
        .put_role_policy()
        .role_name(ROLE_NAME)
        .policy_name(POLICY_NAME)
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .send()
        .await
        .expect("Failed to put role policy");

    client
        .delete_role_policy()
        .role_name(ROLE_NAME)
        .policy_name(POLICY_NAME)
        .send()
        .await
        .expect("Failed to delete role policy");
}
