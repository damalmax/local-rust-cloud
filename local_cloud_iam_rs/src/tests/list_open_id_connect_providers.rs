#[tokio::test]
async fn list_open_id_connect_providers() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .create_open_id_connect_provider()
        .url("https://server1.example.com")
        .thumbprint_list("c3768084dfb3d2b68b7897bf5f565da8eEXAMPLE")
        .client_id_list("my-application-ID")
        .send()
        .await
        .expect("Failed to create OpenID connect provider");

    client
        .create_open_id_connect_provider()
        .url("https://server2.example.com")
        .thumbprint_list("c3768084dfb3d2b68b7897bf5f565da8eEXAMPLE")
        .client_id_list("my-application-ID")
        .send()
        .await
        .expect("Failed to create OpenID connect provider");

    let response = client
        .list_open_id_connect_providers()
        .send()
        .await
        .expect("Failed to get a list of OpenID connect providers");

    assert!(!response.open_id_connect_provider_list().is_empty());
    assert_eq!(response.open_id_connect_provider_list().len(), 2);

    ctx.stop_server().await;
}

#[tokio::test]
async fn list_open_id_connect_providers_empty() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let response = client
        .list_open_id_connect_providers()
        .send()
        .await
        .expect("Failed to get a list of OpenID connect providers");

    assert!(response.open_id_connect_provider_list().is_empty());

    ctx.stop_server().await;
}
