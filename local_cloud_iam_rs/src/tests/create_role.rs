use local_cloud_testing::assertions::assert_not_empty;

use crate::tests::fixture::{tag, CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY, CREATE_USER_PERMISSIONS_BOUNDARY};

#[tokio::test]
async fn create_role() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
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

    let response = client
        .create_role()
        .role_name("Test-Role")
        .path("/")
        .permissions_boundary(policy_output.policy.unwrap().arn().unwrap())
        .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
        .tags(tag("create-user-key1", "create-user-value1"))
        .tags(tag("create-user-key2", "create-user-value2"))
        .tags(tag("create-user-key2", "create-user-value3"))
        .send()
        .await
        .expect("Failed to create IAM role");

    assert!(response.role().is_some());
    let role = response.role().unwrap();
    assert_eq!(role.tags().len(), 3);
    assert_not_empty(role.path());
    assert_not_empty(role.arn());
    assert_not_empty(role.role_name());

    ctx.stop_server().await;
}
