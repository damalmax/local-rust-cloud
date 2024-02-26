use crate::tests::fixture::tag;
use local_cloud_testing::assertions::assert_not_empty;

#[tokio::test]
async fn list_instance_profile_tags_with_marker() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;

    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .create_instance_profile()
        .path("/")
        .instance_profile_name("instance-profile-1")
        .tags(tag("key-1", "value1"))
        .tags(tag("key-2", "value2"))
        .tags(tag("key-3", "value3"))
        .tags(tag("key-4", "value4"))
        .tags(tag("key-5", "value5"))
        .send()
        .await
        .expect("Failed to create IAM instance profile");

    let result = client
        .list_instance_profile_tags()
        .instance_profile_name("instance-profile-1")
        .max_items(3)
        .send()
        .await
        .expect("Failed to get a list of IAM instance profile tags");

    assert!(!result.tags().is_empty());
    assert_eq!(result.tags().len(), 3);
    assert!(result.is_truncated());
    assert_not_empty(result.marker());

    // requesting second page
    let result = client
        .list_instance_profile_tags()
        .instance_profile_name("instance-profile-1")
        .marker(result.marker().unwrap())
        .send()
        .await
        .expect("Failed to get a list of IAM instance profile tags");

    assert!(!result.tags().is_empty());
    assert_eq!(result.tags().len(), 2);
    assert!(!result.is_truncated());
    assert!(result.marker().is_none());
    ctx.stop_server().await;
}
