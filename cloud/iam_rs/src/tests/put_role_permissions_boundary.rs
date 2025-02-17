use crate::tests::fixture::{CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY, CREATE_USER_PERMISSIONS_BOUNDARY};

const ROLE_NAME: &str = "test-role1";

#[tokio::test]
async fn put_role_permissions_boundary() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
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

    let policy_arn = create_policy_output.policy().unwrap().arn().unwrap();

    client
        .create_role()
        .role_name(ROLE_NAME)
        .path("/")
        .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
        .send()
        .await
        .expect("Failed to create IAM role");

    client
        .put_role_permissions_boundary()
        .role_name(ROLE_NAME)
        .permissions_boundary(policy_arn)
        .send()
        .await
        .expect("Failed to put permissions boundary to IAM role");
}

#[tokio::test]
async fn put_role_permissions_boundary_policy_does_not_exist() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let policy_arn = "arn:aws:iam::000000000001:policy/unknown";

    client
        .create_role()
        .role_name(ROLE_NAME)
        .path("/")
        .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
        .send()
        .await
        .expect("Failed to create IAM role");

    let response = client
        .put_role_permissions_boundary()
        .role_name(ROLE_NAME)
        .permissions_boundary(policy_arn)
        .send()
        .await;

    assert!(response.is_err());
    let sdk_error = response.unwrap_err();
    assert_eq!(404u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_no_such_entity_exception());
    assert_eq!("NoSuchEntity", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(
        error.meta().message().unwrap(),
        "IAM policy with ARN 'arn:aws:iam::000000000001:policy/unknown' doesn't exist."
    );
}

#[tokio::test]
async fn put_role_permissions_boundary_role_does_not_exist() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
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

    let policy_arn = create_policy_output.policy().unwrap().arn().unwrap();

    let response = client
        .put_role_permissions_boundary()
        .role_name(ROLE_NAME)
        .permissions_boundary(policy_arn)
        .send()
        .await;

    assert!(response.is_err());
    let sdk_error = response.unwrap_err();
    assert_eq!(404u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_no_such_entity_exception());
    assert_eq!("NoSuchEntity", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(error.meta().message().unwrap(), "IAM role with name 'test-role1' doesn't exist.");
}
