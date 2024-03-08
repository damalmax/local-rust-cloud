use aws_sdk_iam::types::PermissionsBoundaryAttachmentType;

use local_cloud_testing::assertions::assert_not_empty;

use crate::tests::fixture::{tag, CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY, CREATE_USER_PERMISSIONS_BOUNDARY};

const ROLE_NAME: &str = "Test-Role";
const ROLE_DESCRIPTION: &str = "Role description";

#[tokio::test]
async fn get_role() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_policy_output = super::fixture::create_policy(
        &client,
        "test-policy",
        "user-test-policy",
        "/division_abc/subdivision_xyz/",
        CREATE_USER_PERMISSIONS_BOUNDARY,
        None,
    )
    .await
    .unwrap();

    let permissions_boundary_arn = create_policy_output.policy().unwrap().arn().unwrap();
    let create_role_output = client
        .create_role()
        .role_name(ROLE_NAME)
        .path("/")
        .permissions_boundary(permissions_boundary_arn)
        .description(ROLE_DESCRIPTION)
        .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
        .tags(tag("create-key1", "create-user-value1"))
        .tags(tag("create-key2", "create-user-value2"))
        .tags(tag("create-key3", "create-user-value3"))
        .send()
        .await
        .expect("Failed to create IAM role");

    let result = client
        .get_role()
        .role_name(ROLE_NAME)
        .send()
        .await
        .expect("Failed to get IAM role");

    let role = result.role().unwrap();

    assert!(role.tags().is_empty());
    assert_not_empty(role.role_name());
    assert_not_empty(role.role_id());
    assert_eq!(role.max_session_duration().unwrap(), 3600);
    assert!(role.permissions_boundary().is_some());
    let permissions_boundary = role.permissions_boundary().unwrap();
    assert_not_empty(permissions_boundary.permissions_boundary_arn());
    assert!(permissions_boundary.permissions_boundary_type().is_some());
    assert_eq!(permissions_boundary.permissions_boundary_type().unwrap(), &PermissionsBoundaryAttachmentType::Policy);
    assert_eq!(role.path(), "/");
    assert_eq!(role.arn(), create_role_output.role().unwrap().arn());
    assert_eq!(role.description().unwrap(), ROLE_DESCRIPTION);
}

#[tokio::test]
async fn get_role_does_not_exist() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let response = client.get_role().role_name(ROLE_NAME).send().await;

    assert!(response.is_err());
    let sdk_error = response.unwrap_err();
    assert_eq!(404u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_no_such_entity_exception());
    assert_eq!("NoSuchEntity", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(error.meta().message().unwrap(), "IAM role with name 'Test-Role' doesn't exist.");
}
