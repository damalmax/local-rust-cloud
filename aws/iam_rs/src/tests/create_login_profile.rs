use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;

#[tokio::test]
async fn create_login_profile() {
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
    .expect("Failed to create IAM policy");

    super::fixture::create_user(&client, "SuperUser", "/", policy_output.policy.unwrap().arn(), None)
        .await
        .expect("Failed to create IAM user");

    let response = client
        .create_login_profile()
        .password_reset_required(true)
        .user_name("SuperUser")
        .password("fvrmweioyt43y8989JK9*(&#@*(")
        .send()
        .await
        .expect("Failed to create login profile");

    assert!(response.login_profile().is_some());
}

#[tokio::test]
async fn create_login_profile_login_profile_already_exists() {
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
    .expect("Failed to create IAM policy");

    super::fixture::create_user(&client, "SuperUser", "/", policy_output.policy.unwrap().arn(), None)
        .await
        .expect("Failed to create IAM user");

    // first attempt should be successful
    let response = client
        .create_login_profile()
        .password_reset_required(true)
        .user_name("SuperUser")
        .password("fvrmweioyt43y8989JK9*(&#@*(")
        .send()
        .await
        .expect("Failed to create login profile");
    assert!(response.login_profile().is_some());

    // second
    let response = client
        .create_login_profile()
        .password_reset_required(true)
        .user_name("SuperUser")
        .password("fvrmweioyt43y8989JK9*(&#@*(")
        .send()
        .await;

    assert!(response.is_err());

    let sdk_error = response.unwrap_err();
    assert_eq!(409u16, sdk_error.raw_response().unwrap().status().as_u16());
}
