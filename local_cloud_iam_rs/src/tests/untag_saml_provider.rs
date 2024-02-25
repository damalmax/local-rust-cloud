use data_encoding::BASE64;

use crate::tests::fixture::tag;

#[tokio::test]
async fn untag_saml_provider() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_saml_provider_output = client
        .create_saml_provider()
        .saml_metadata_document(BASE64.encode(include_str!("resources/saml-metadata.xml").as_bytes()))
        .name("MyUniversity")
        .tags(tag("key-1", "value1"))
        .tags(tag("key-2", "value2"))
        .tags(tag("key-3", "value3"))
        .tags(tag("key-4", "value4"))
        .tags(tag("key-5", "value5"))
        .send()
        .await
        .expect("Failed to create IAM SAML provider");

    let saml_provider_arn = create_saml_provider_output.saml_provider_arn().unwrap();

    client
        .untag_saml_provider()
        .saml_provider_arn(saml_provider_arn)
        .tag_keys("key-1")
        .tag_keys("key-2")
        .send()
        .await
        .expect("Failed to untag IAM SAML provider");

    let tags_output = client
        .list_saml_provider_tags()
        .saml_provider_arn(saml_provider_arn)
        .send()
        .await
        .expect("Failed to get a list of tags for IAM SAML provider");

    assert_eq!(tags_output.tags().len(), 3);

    ctx.stop_server().await;
}
