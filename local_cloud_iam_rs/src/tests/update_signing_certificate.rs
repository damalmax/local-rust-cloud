use aws_sdk_iam::types::StatusType;

const USER_NAME: &str = "TestUser1";

#[tokio::test]
async fn update_signing_certificate() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    super::fixture::create_user(&client, USER_NAME, "/", None, None)
        .await
        .expect("Failed to create IAM user");

    let upload_signing_certificate_output = client
        .upload_signing_certificate()
        .certificate_body(include_str!("resources/cert.pem").trim())
        .user_name(USER_NAME)
        .send()
        .await
        .expect("Failed to upload signing certificate");

    client
        .update_signing_certificate()
        .certificate_id(
            upload_signing_certificate_output
                .certificate()
                .unwrap()
                .certificate_id(),
        )
        .user_name(USER_NAME)
        .status(StatusType::Inactive)
        .send()
        .await
        .expect("Failed to update signing certificate");

    ctx.stop_server().await;
}
