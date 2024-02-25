use crate::tests::fixture::tag;

const INSTANCE_PROFILE_NAME: &str = "instance-profile-1";

#[tokio::test]
async fn untag_instance_profile() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .create_instance_profile()
        .path("/")
        .instance_profile_name(INSTANCE_PROFILE_NAME)
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .send()
        .await
        .expect("Failed to create IAM instance profile");

    client
        .untag_instance_profile()
        .instance_profile_name(INSTANCE_PROFILE_NAME)
        .tag_keys("key1")
        .tag_keys("key2")
        .send()
        .await
        .expect("Failed to untag IAM instance profile");

    let tags_output = client
        .list_instance_profile_tags()
        .instance_profile_name(INSTANCE_PROFILE_NAME)
        .send()
        .await
        .expect("Failed to get a list of tags for IAM instance profile");

    assert!(tags_output.tags().is_empty());

    ctx.stop_server().await;
}
