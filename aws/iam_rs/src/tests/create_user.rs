use testing::assertions::assert_not_empty;

use crate::tests::fixture::{tag, CREATE_USER_PERMISSIONS_BOUNDARY};

#[tokio::test]
async fn create_user() {
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

    let create_user_output = super::fixture::create_user(
        &client,
        "user1",
        "/",
        policy_output.policy.unwrap().arn(),
        Some(vec![
            tag("create-user-key1", "create-user-value1"),
            tag("create-user-key2", "create-user-value2"),
            tag("create-user-key2", "create-user-value3"),
        ]),
    )
    .await
    .expect("Failed to create IAM user");

    assert!(create_user_output.user().is_some());
    let user = create_user_output.user().unwrap();
    assert_eq!(user.tags().len(), 3);
    assert_not_empty(user.path());
    assert_not_empty(user.arn());
    assert_not_empty(user.user_name());
    assert_not_empty(user.user_id());
}
