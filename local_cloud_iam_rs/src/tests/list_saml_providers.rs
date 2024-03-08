use data_encoding::BASE64;

#[tokio::test]
async fn list_saml_providers() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .create_saml_provider()
        .saml_metadata_document(BASE64.encode(include_str!("resources/saml-metadata.xml").as_bytes()))
        .name("MyUniversity")
        .send()
        .await
        .expect("Failed to create IAM SAML provider");

    let result = client
        .list_saml_providers()
        .send()
        .await
        .expect("Failed to get a list of SAML providers");

    assert_eq!(result.saml_provider_list().len(), 1);
}

#[tokio::test]
async fn list_saml_providers_empty() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let result = client
        .list_saml_providers()
        .send()
        .await
        .expect("Failed to get a list of SAML providers");

    assert!(result.saml_provider_list().is_empty());
}
