use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_iam::{config::Region, types::Tag};

use super::*;

#[actix_rt::test]
async fn create_policy() {
    let mut ctx = TEST_SUITE.create_test_ctx().await;
    let port = ctx.port;
    let config = aws_config::SdkConfig::builder()
        .region(Some(Region::new("eu-local-1")))
        .endpoint_url(format!("http://localhost:{}/iam", port))
        .credentials_provider(SharedCredentialsProvider::new(credentials_provider()))
        .build();
    let client = aws_sdk_iam::Client::new(&config);

    let response = client
        .create_policy()
        .description("policy-description")
        .path("/")
        .policy_document(include_str!("resources/create_user__permissions_boundary.json"))
        .policy_name("some-policy-name")
        .tags(Tag::builder().key("key1").value("value1").build())
        .tags(Tag::builder().key("key2").value("value2").build())
        .tags(Tag::builder().key("key3").value("value3").build())
        .send()
        .await
        .expect("Failed to create IAM policy");

    assert!(response.policy().is_some());
    assert!(response.policy().unwrap().tags().unwrap().len() > 0);

    ctx.stop_server().await;
}

#[actix_rt::test]
async fn create_policy_too_many_tags() {
    let mut ctx = TEST_SUITE.create_test_ctx().await;
    let port = ctx.port;
    let config = aws_config::SdkConfig::builder()
        .region(Some(Region::new("eu-local-1")))
        .endpoint_url(format!("http://localhost:{}/iam", port))
        .credentials_provider(SharedCredentialsProvider::new(credentials_provider()))
        .build();
    let client = aws_sdk_iam::Client::new(&config);

    let tags = (0..51)
        .map(|i| Tag::builder().key(format!("key-{}", i)).value(format!("value-{}", i)).build())
        .collect();
    let request_builder = client
        .create_policy()
        .description("policy-description")
        .path("/")
        .policy_document(include_str!("resources/create_user__permissions_boundary.json"))
        .policy_name("some-policy-name")
        .set_tags(Option::Some(tags));

    let result = request_builder.send().await;
    assert!(result.is_err());

    let sdk_error = result.unwrap_err();
    assert_eq!(400u16, sdk_error.raw_response().unwrap().http().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_invalid_input_exception());
    assert_eq!("InvalidInput", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert!(error.meta().message().unwrap().starts_with("1 validation error detected:"));

    ctx.stop_server().await;
}
