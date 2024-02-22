use crate::tests::fixture::tag;

#[actix_rt::test]
async fn untag_open_id_connect_provider() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_open_id_connect_provider_output = client
        .create_open_id_connect_provider()
        .url("https://server.example.com")
        .thumbprint_list("c3768084dfb3d2b68b7897bf5f565da8eEXAMPLE")
        .client_id_list("my-application-ID")
        .tags(tag("key-1", "value-1"))
        .tags(tag("key-2", "value-2"))
        .tags(tag("key-3", "value-3"))
        .tags(tag("key-4", "value-4"))
        .tags(tag("key-5", "value-5"))
        .send()
        .await
        .unwrap();

    let provider_arn = create_open_id_connect_provider_output
        .open_id_connect_provider_arn()
        .unwrap();

    client
        .untag_open_id_connect_provider()
        .open_id_connect_provider_arn(provider_arn)
        .tag_keys("key-1")
        .tag_keys("key-2")
        .send()
        .await
        .expect("Failed to untag IAM OpenID Connect provider");

    let tags_output = client
        .list_open_id_connect_provider_tags()
        .open_id_connect_provider_arn(provider_arn)
        .send()
        .await
        .expect("Failed to get a list of tags for IAM OpenID Connect provider");

    assert_eq!(tags_output.tags().len(), 3);

    ctx.stop_server().await;
}
