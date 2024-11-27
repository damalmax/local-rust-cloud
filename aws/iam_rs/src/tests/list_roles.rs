use crate::tests::fixture::{tag, CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY, CREATE_USER_PERMISSIONS_BOUNDARY};

#[tokio::test]
async fn list_roles_empty() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let roles = client
        .list_roles()
        .send()
        .await
        .expect("Failed to get a list of IAM roles");

    assert!(roles.roles().is_empty());
}

#[tokio::test]
async fn list_roles() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
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

    let policy_arn = policy_output.policy().unwrap().arn().unwrap();

    client
        .create_role()
        .role_name("Test-Role")
        .path("/")
        .permissions_boundary(policy_arn)
        .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
        .tags(tag("create-role-key2", "create-role-value3"))
        .send()
        .await
        .expect("Failed to create IAM role");

    client
        .create_role()
        .role_name("Test-Role2")
        .path("/")
        .permissions_boundary(policy_arn)
        .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
        .tags(tag("create-role-key1", "create-role-value1"))
        .send()
        .await
        .expect("Failed to create IAM role");

    let roles = client
        .list_roles()
        .send()
        .await
        .expect("Failed to get a list of IAM roles");

    assert_eq!(roles.roles().len(), 2);
}
