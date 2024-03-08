const CLIENT_ID: &str = "my-application-ID";

#[tokio::test]
async fn remove_client_id_from_open_id_connect_provider() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_open_id_connect_provider_output = client
        .create_open_id_connect_provider()
        .url("https://server.example.com")
        .thumbprint_list("c3768084dfb3d2b68b7897bf5f565da8eEXAMPLE")
        .client_id_list(CLIENT_ID)
        .send()
        .await
        .expect("Failed to create OpenID connect provider");

    let arn = create_open_id_connect_provider_output
        .open_id_connect_provider_arn()
        .unwrap();

    client
        .remove_client_id_from_open_id_connect_provider()
        .open_id_connect_provider_arn(arn)
        .client_id(CLIENT_ID)
        .send()
        .await
        .expect("Failed to remove client ID from OpenID connect provider");
}
