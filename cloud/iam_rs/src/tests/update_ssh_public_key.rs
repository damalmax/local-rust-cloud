use aws_sdk_iam::types::StatusType;

const USER_NAME: &str = "test-user1";

#[tokio::test]
async fn update_ssh_public_key() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
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

    let key_id = response.ssh_public_key().unwrap().ssh_public_key_id();
    let ssh_public_key = response.ssh_public_key().unwrap().status();

    client
        .update_ssh_public_key()
        .user_name(USER_NAME)
        .ssh_public_key_id(key_id)
        .status(StatusType::Inactive)
        .send()
        .await
        .expect("Failed to update SSH public key");
}
