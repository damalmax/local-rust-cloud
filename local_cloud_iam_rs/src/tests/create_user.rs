use local_cloud_testing::assertions::assert_not_empty;

use crate::tests::fixture::{tag, CREATE_USER_PERMISSIONS_BOUNDARY};

#[actix_rt::test]
async fn create_user() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
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
        .create_user()
        .user_name("user1")
        .path("/")
        .permissions_boundary(policy_output.policy.unwrap().arn().unwrap())
        .tags(tag("create-user-key1", "create-user-value1"))
        .tags(tag("create-user-key2", "create-user-value2"))
        .tags(tag("create-user-key2", "create-user-value3"))
        .send()
        .await
        .expect("Failed to create IAM policy");

    assert!(response.user().is_some());
    let user = response.user().unwrap();
    assert_eq!(user.tags().len(), 3);
    assert_not_empty(user.path());
    assert_not_empty(user.arn());
    assert_not_empty(user.user_name());

    ctx.stop_server().await;
}
