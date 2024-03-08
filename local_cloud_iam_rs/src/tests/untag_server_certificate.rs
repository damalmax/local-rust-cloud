use crate::tests::fixture::tag;

const CERTIFICATE_NAME: &str = "Test-Certificate-1";

#[tokio::test]
async fn untag_server_certificate() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .upload_server_certificate()
        .server_certificate_name(CERTIFICATE_NAME)
        .path("/servers/")
        .certificate_body(include_str!("resources/cert.pem"))
        .private_key(include_str!("resources/key.pem"))
        .tags(tag("key-1", "value-1"))
        .tags(tag("key-2", "value-2"))
        .tags(tag("key-3", "value-3"))
        .tags(tag("key-4", "value-4"))
        .tags(tag("key-5", "value-5"))
        .send()
        .await
        .expect("Failed to upload server certificate");

    client
        .untag_server_certificate()
        .server_certificate_name(CERTIFICATE_NAME)
        .tag_keys("key-1")
        .tag_keys("key-2")
        .send()
        .await
        .expect("Failed to untag IAM server certificate");

    let tags_output = client
        .list_server_certificate_tags()
        .server_certificate_name(CERTIFICATE_NAME)
        .send()
        .await
        .expect("Failed to get a list of tags for IAM server certificate");

    assert_eq!(tags_output.tags().len(), 3);
}
