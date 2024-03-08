use local_cloud_testing::assertions::assert_not_empty;

use crate::tests::fixture::tag;

#[tokio::test]
async fn test_create_instance_profile() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let response = client
        .create_instance_profile()
        .path("/")
        .instance_profile_name("instance-profile-1")
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .send()
        .await
        .unwrap();

    assert!(response.instance_profile().is_some());
    let instance_profile = response.instance_profile().unwrap();
    assert_eq!(instance_profile.tags().len(), 2);
    assert_not_empty(instance_profile.path());
    assert_eq!(instance_profile.path(), "/");
    assert!(instance_profile.roles().is_empty());
    assert_not_empty(instance_profile.instance_profile_name());
    assert_eq!(instance_profile.instance_profile_name(), "instance-profile-1");
}

#[tokio::test]
async fn test_create_instance_profile_without_path() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let response = client
        .create_instance_profile()
        .instance_profile_name("instance-profile-1")
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .send()
        .await
        .unwrap();

    assert!(response.instance_profile().is_some());
    let instance_profile = response.instance_profile().unwrap();
    assert_eq!(instance_profile.tags().len(), 2);
    assert_not_empty(instance_profile.path());
    assert_eq!(instance_profile.path(), "/");
    assert!(instance_profile.roles().is_empty());
    assert_not_empty(instance_profile.instance_profile_name());
    assert_eq!(instance_profile.instance_profile_name(), "instance-profile-1");
}
