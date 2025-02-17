use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;

const GROUP_NAME: &str = "test_group_1";

#[tokio::test]
async fn detach_group_policy() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let _create_group_output = client
        .create_group()
        .group_name(GROUP_NAME)
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

    let policy_arn = policy_output.policy().unwrap().arn().unwrap();

    client
        .attach_group_policy()
        .group_name(GROUP_NAME)
        .policy_arn(policy_arn)
        .send()
        .await
        .expect("Failed to attach group policy");

    client
        .detach_group_policy()
        .group_name(GROUP_NAME)
        .policy_arn(policy_arn)
        .send()
        .await
        .expect("Failed to detach group policy");
}
