use testing::assertions::assert_not_empty;

use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;

#[tokio::test]
async fn get_group_policy() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .create_group()
        .group_name("test_group_1")
        .path("/")
        .send()
        .await
        .expect("Failed to create IAM group");

    client
        .put_group_policy()
        .group_name("test_group_1")
        .policy_name("test_policy_1")
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .send()
        .await
        .expect("Failed to put IAM group policy");

    let result = client
        .get_group_policy()
        .group_name("test_group_1")
        .policy_name("test_policy_1")
        .send()
        .await;

    assert!(result.is_ok());

    let output = result.unwrap();
    assert_eq!(output.group_name(), "test_group_1");
    assert_eq!(output.policy_name(), "test_policy_1");
    assert_not_empty(output.policy_document());
}
