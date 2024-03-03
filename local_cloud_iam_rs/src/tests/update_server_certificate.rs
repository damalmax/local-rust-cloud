const SERVER_CERTIFICATE_NAME: &str = "test-cert-1";
const NEW_SERVER_CERTIFICATE_NAME: &str = "test-cert-3";

#[tokio::test]
async fn update_server_certificate() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .upload_server_certificate()
        .server_certificate_name(SERVER_CERTIFICATE_NAME)
        .path("/servers/")
        .certificate_body(include_str!("resources/cert.pem"))
        .private_key(include_str!("resources/key.pem"))
        .send()
        .await
        .expect("Failed to upload server certificate");

    client
        .update_server_certificate()
        .server_certificate_name(SERVER_CERTIFICATE_NAME)
        .new_server_certificate_name(NEW_SERVER_CERTIFICATE_NAME)
        .new_path("/sector/")
        .send()
        .await
        .expect("Failed to update server certificate");

    ctx.stop_server().await;
}
