use crate::tests::fixture::tag;

#[tokio::test]
async fn get_open_id_connect_provider() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_open_id_connect_provider_output = client
        .create_open_id_connect_provider()
        .url("https://server.example.com")
        .thumbprint_list("c3768084dfb3d2b68b7897bf5f565da8eEXAMPLE")
        .client_id_list("my-application-ID")
        .tags(tag("key-1", "value1"))
        .tags(tag("key-2", "value2"))
        .send()
        .await
        .expect("Failed to create OpenID connect provider");

    let arn = create_open_id_connect_provider_output
        .open_id_connect_provider_arn()
        .unwrap();

    let response = client
        .get_open_id_connect_provider()
        .open_id_connect_provider_arn(arn)
        .send()
        .await
        .expect("Failed to get OpenID connect provider");

    assert_eq!(response.url().unwrap(), "https://server.example.com");
    assert_eq!(response.client_id_list().len(), 1);
    assert_eq!(response.thumbprint_list().len(), 1);
    assert!(response.create_date().is_some());
    assert!(response.tags().is_empty());
}
