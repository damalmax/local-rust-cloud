use testing::assertions::assert_not_empty;

#[tokio::test]
async fn create_group() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let response = client
        .create_group()
        .group_name("test_group_1")
        .path("/")
        .send()
        .await
        .expect("Failed to create IAM group");

    assert!(response.group().is_some());
    let group = response.group().unwrap();
    assert_not_empty(group.path());
    assert_not_empty(group.arn());
    assert_not_empty(group.group_id());
    assert_not_empty(group.group_name());
}
