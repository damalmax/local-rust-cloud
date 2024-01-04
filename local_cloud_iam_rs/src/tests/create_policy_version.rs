use crate::tests::{credentials_provider, TEST_SUITE};
use aws_config::BehaviorVersion;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_iam::{config::Region, types::Tag};

#[actix_rt::test]
async fn create_policy_version() {
    let mut ctx = TEST_SUITE.create_test_ctx().await;
    let port = ctx.port;
    let config = aws_config::SdkConfig::builder()
        .region(Some(Region::new("eu-local-1")))
        .endpoint_url(format!("http://localhost:{}/iam", port))
        .credentials_provider(SharedCredentialsProvider::new(credentials_provider()))
        .behavior_version(BehaviorVersion::latest())
        .build();
    let client = aws_sdk_iam::Client::new(&config);

    let response = client
        .create_policy()
        .description("policy-description")
        .path("/")
        .policy_document(include_str!("resources/create_user__permissions_boundary.json"))
        .policy_name("some-policy-name")
        .tags(Tag::builder().key("key1").value("value1").build().unwrap())
        .tags(Tag::builder().key("key2").value("value2").build().unwrap())
        .tags(Tag::builder().key("key3").value("value3").build().unwrap())
        .send()
        .await
        .expect("Failed to create IAM policy");

    let create_policy_version_response = client
        .create_policy_version()
        .policy_arn(response.policy.unwrap().arn.unwrap())
        .policy_document(include_str!("resources/create_user__permissions_boundary.json"))
        .set_as_default(false)
        .send()
        .await
        .expect("Failed to create IAM policy version");

    assert!(create_policy_version_response.policy_version().is_some());

    ctx.stop_server().await;
}
