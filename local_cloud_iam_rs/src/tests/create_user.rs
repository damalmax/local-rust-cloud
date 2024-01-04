use crate::tests::credentials_provider;
use aws_config::BehaviorVersion;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_iam::{config::Region, types::Tag};

#[actix_rt::test]
async fn create_user() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = aws_config::SdkConfig::builder()
        .region(Some(Region::new("eu-local-1")))
        .endpoint_url(format!("http://localhost:{}/iam", port))
        .credentials_provider(SharedCredentialsProvider::new(credentials_provider()))
        .behavior_version(BehaviorVersion::latest())
        .build();
    let client = aws_sdk_iam::Client::new(&config);

    let response = client
        .create_user()
        .user_name("user1")
        .path("/")
        .permissions_boundary("")
        .tags(Tag::builder().key("key1").value("value1").build().unwrap())
        .tags(Tag::builder().key("key2").value("value2").build().unwrap())
        .tags(Tag::builder().key("key2").value("value3").build().unwrap())
        .send()
        .await
        .expect("Failed to create IAM policy");

    assert!(response.user().is_some());

    ctx.stop_server().await;
}
