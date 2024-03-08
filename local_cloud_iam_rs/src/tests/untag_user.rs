use crate::tests::fixture::tag;

const USER_NAME: &str = "test-user-1";

#[tokio::test]
async fn untag_user() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    super::fixture::create_user(
        &client,
        USER_NAME,
        "/",
        None,
        Some(vec![
            tag("key-1", "value-1"),
            tag("key-2", "value-2"),
            tag("key-3", "value-3"),
            tag("key-4", "value-4"),
            tag("key-5", "value-5"),
        ]),
    )
    .await
    .expect("Failed to create IAM user");

    client
        .untag_user()
        .user_name(USER_NAME)
        .tag_keys("key-1")
        .tag_keys("key-2")
        .send()
        .await
        .expect("Failed to untag IAM user");

    let tags_output = client
        .list_user_tags()
        .user_name(USER_NAME)
        .send()
        .await
        .expect("Failed to get a list of tags for IAM user");

    assert_eq!(tags_output.tags().len(), 3);
}
