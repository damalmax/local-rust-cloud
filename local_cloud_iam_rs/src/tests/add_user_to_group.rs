use crate::tests::fixture::{tag, CREATE_USER_PERMISSIONS_BOUNDARY};

#[tokio::test]
async fn add_user_to_group() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let _create_group_output = client
        .create_group()
        .group_name("test_group_1")
        .path("/")
        .send()
        .await
        .expect("Failed to create IAM group");

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

    let _create_user_output = super::fixture::create_user(
        &client,
        "user1",
        "/",
        policy_output.policy.unwrap().arn(),
        Some(vec![
            tag("create-user-key1", "create user value 1"),
            tag("create-user-key2", "create user value 2"),
            tag("create-user-key3", "create user value 3"),
        ]),
    )
    .await
    .expect("Failed to create IAM user");

    let _response = client
        .add_user_to_group()
        .user_name("user1")
        .group_name("test_group_1")
        .send()
        .await
        .expect("Failed to add user to group");
    ctx.stop_server().await;
}
