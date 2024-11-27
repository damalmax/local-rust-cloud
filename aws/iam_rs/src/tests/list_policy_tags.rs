use crate::tests::fixture::{tag, CREATE_USER_PERMISSIONS_BOUNDARY};
use testing::assertions::assert_not_empty;

#[tokio::test]
async fn list_policy_tags_with_marker() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let created_policy = super::fixture::create_policy(
        &client,
        "some-policy-name",
        "some-policy-description",
        "/",
        CREATE_USER_PERMISSIONS_BOUNDARY,
        Some(vec![
            tag("tag-key1", "tag-value1"),
            tag("tag-key2", "tag-value2"),
            tag("tag-key3", "tag-value3"),
            tag("tag-key4", "tag-value4"),
            tag("tag-key5", "tag-value5"),
            tag("tag-key6", "tag-value6"),
        ]),
    )
    .await
    .unwrap();

    assert!(created_policy.policy().is_some());

    let policy_arn = created_policy.policy().unwrap().arn().unwrap();

    let result = client
        .list_policy_tags()
        .policy_arn(policy_arn)
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
        .list_policy_tags()
        .max_items(3)
        .policy_arn(policy_arn)
        .marker(result.marker().unwrap())
        .send()
        .await
        .expect("Failed to get a list of IAM policy tags");

    assert!(!result.tags().is_empty());
    assert_eq!(result.tags().len(), 3);
    assert!(!result.is_truncated());
    assert!(result.marker().is_none());
}
