use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;

#[tokio::test]
async fn put_group_policy() {
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

    let result = client
        .put_group_policy()
        .group_name("test_group_1")
        .policy_name("test_policy_1")
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .send()
        .await;

    assert!(result.is_ok());
}
