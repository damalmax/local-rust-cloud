use crate::tests::fixture::tag;
use local_cloud_testing::assertions::assert_not_empty;

#[tokio::test]
async fn list_user_tags_with_marker() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_user_output = super::fixture::create_user(
        &client,
        "user123",
        "/",
        None,
        Some(vec![
            tag("create-user-key1", "create-user-value1"),
            tag("create-user-key2", "create-user-value2"),
            tag("create-user-key3", "create-user-value3"),
            tag("create-user-key4", "create-user-value4"),
            tag("create-user-key5", "create-user-value5"),
            tag("create-user-key6", "create-user-value6"),
        ]),
    )
    .await
    .expect("Failed to create IAM user");

    assert!(create_user_output.user().is_some());

    let result = client
        .list_user_tags()
        .user_name("user123")
        .max_items(3)
        .send()
        .await
        .expect("Failed to get a list of IAM policy tags");

    assert!(!result.tags().is_empty());
    assert_eq!(result.tags().len(), 3);
    assert!(result.is_truncated());
    assert_not_empty(result.marker());

    // requesting second page
    let result = client
        .list_user_tags()
        .user_name("user123")
        .marker(result.marker().unwrap())
        .send()
        .await
        .expect("Failed to get a list of IAM policy tags");

    assert!(!result.tags().is_empty());
    assert_eq!(result.tags().len(), 3);
    assert!(!result.is_truncated());
    assert!(result.marker().is_none());
}
