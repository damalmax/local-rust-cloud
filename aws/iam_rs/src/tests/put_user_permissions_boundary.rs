use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;

const USER_NAME: &str = "user1";
#[tokio::test]
async fn put_user_permissions_boundary() {
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
        .create_user()
        .user_name(USER_NAME)
        .path("/")
        .send()
        .await
        .expect("Failed to create IAM user");

    client
        .put_user_permissions_boundary()
        .user_name(USER_NAME)
        .permissions_boundary(policy_arn)
        .send()
        .await
        .expect("Failed to put permissions boundary to IAM user");
}

#[tokio::test]
async fn put_user_permissions_boundary_policy_does_not_exist() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let policy_arn = "arn:aws:iam::000000000001:policy/unknown";

    client
        .create_user()
        .user_name(USER_NAME)
        .path("/")
        .send()
        .await
        .expect("Failed to create IAM user");

    let response = client
        .put_user_permissions_boundary()
        .user_name(USER_NAME)
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
async fn put_user_permissions_boundary_user_does_not_exist() {
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
        .put_user_permissions_boundary()
        .user_name(USER_NAME)
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
    assert_eq!(error.meta().message().unwrap(), "IAM user with name 'user1' doesn't exist.");
}
