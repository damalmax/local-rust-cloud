use data_encoding::BASE64;
use testing::assertions::assert_not_empty;

use crate::tests::fixture::tag;

#[tokio::test]
async fn list_saml_provider_tags() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_saml_provider_output = client
        .create_saml_provider()
        .saml_metadata_document(BASE64.encode(include_str!("resources/saml-metadata.xml").as_bytes()))
        .name("MyUniversity")
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .tags(tag("key3", "value3"))
        .tags(tag("key4", "value4"))
        .tags(tag("key5", "value5"))
        .send()
        .await
        .expect("Failed to create IAM SAML provider");

    let arn = create_saml_provider_output.saml_provider_arn().unwrap();

    let result = client
        .list_saml_provider_tags()
        .saml_provider_arn(arn)
        .max_items(3)
        .send()
        .await
        .expect("Failed to get a list of IAM SAML provider tags");

    assert!(!result.tags().is_empty());
    assert_eq!(result.tags().len(), 3);
    assert!(result.is_truncated());
    assert_not_empty(result.marker());

    // requesting second page
    let result = client
        .list_saml_provider_tags()
        .saml_provider_arn(arn)
        .marker(result.marker().unwrap())
        .send()
        .await
        .expect("Failed to get a list of IAM SAML provider tags");

    assert!(!result.tags().is_empty());
    assert_eq!(result.tags().len(), 2);
    assert!(!result.is_truncated());
    assert!(result.marker().is_none());
}
