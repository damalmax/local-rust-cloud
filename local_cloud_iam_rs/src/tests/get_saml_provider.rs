use data_encoding::BASE64;

use local_cloud_testing::assertions::assert_not_empty;

use crate::tests::fixture::tag;

#[tokio::test]
async fn get_saml_provider() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_saml_provider_output = client
        .create_saml_provider()
        .saml_metadata_document(BASE64.encode(include_str!("resources/saml-metadata.xml").as_bytes()))
        .name("MyUniversity")
        .tags(tag("key-1", "value1"))
        .tags(tag("key-2", "value2"))
        .tags(tag("key-3", "value3"))
        .send()
        .await
        .expect("Failed to create IAM SAML provider");

    let arn = create_saml_provider_output.saml_provider_arn().unwrap();

    let result = client
        .get_saml_provider()
        .saml_provider_arn(arn)
        .send()
        .await
        .expect("Failed to get IAM SAML provider");

    assert!(result.tags().is_empty());
    assert_not_empty(result.saml_metadata_document());
    assert!(result.valid_until().is_none());
    assert!(result.create_date().is_some());

    ctx.stop_server().await;
}

#[tokio::test]
async fn get_saml_provider_does_not_exist() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .create_saml_provider()
        .saml_metadata_document(BASE64.encode(include_str!("resources/saml-metadata.xml").as_bytes()))
        .name("MyUniversity")
        .tags(tag("key-1", "value1"))
        .send()
        .await
        .expect("Failed to create IAM SAML provider");

    let arn = "arn:aws:iam::000000000001:saml-provider/MyUniversity2";
    let response = client.get_saml_provider().saml_provider_arn(arn).send().await;

    assert!(response.is_err());
    let sdk_error = response.unwrap_err();
    assert_eq!(404u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_no_such_entity_exception());
    assert_eq!("NoSuchEntity", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(
        error.meta().message().unwrap(),
        "IAM SAML provider with ARN 'arn:aws:iam::000000000001:saml-provider/MyUniversity2' doesn't exist."
    );

    ctx.stop_server().await;
}
