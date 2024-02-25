use crate::tests::fixture::tag;
use data_encoding::BASE64;

#[tokio::test]
async fn create_saml_provider() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let result = client
        .create_saml_provider()
        .saml_metadata_document(BASE64.encode(include_str!("resources/saml-metadata.xml").as_bytes()))
        .name("MyUniversity")
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .tags(tag("key3", "value3"))
        .send()
        .await
        .expect("Failed to create IAM SAML provider");

    assert!(result.saml_provider_arn().is_some());
    assert_eq!(result.saml_provider_arn().unwrap(), "arn:aws:iam::000000000001:saml-provider/MyUniversity");
    assert_eq!(result.tags().len(), 3);

    ctx.stop_server().await;
}
