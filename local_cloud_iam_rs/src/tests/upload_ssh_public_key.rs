use aws_sdk_iam::types::StatusType;

const USER_NAME: &str = "test-user1";

#[actix_rt::test]
async fn upload_ssh_public_key() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    super::fixture::create_user(&client, USER_NAME, "/", None, None)
        .await
        .expect("Failed to create IAM user");

    let response = client
        .upload_ssh_public_key()
        .user_name(USER_NAME)
        .ssh_public_key_body(include_str!("./resources/id_ed25519.pub"))
        .send()
        .await
        .expect("Failed to upload SSH public key");

    assert!(response.ssh_public_key().is_some());

    let ssh_public_key = response.ssh_public_key().unwrap();
    assert_eq!(ssh_public_key.ssh_public_key_body(), include_str!("./resources/id_ed25519.pub").trim());
    assert_eq!(ssh_public_key.user_name(), USER_NAME);
    assert_eq!(ssh_public_key.fingerprint(), "MTR1iTLcl+6KQSEaI28KY1ETJhkHkNGWxozRKuoSSL4");
    assert!(ssh_public_key.ssh_public_key_id().starts_with("APKA"));
    assert_eq!(ssh_public_key.status().as_str(), StatusType::Active.as_str());
    assert_eq!(ssh_public_key.ssh_public_key_id().len(), 21);

    ctx.stop_server().await;
}
