use local_cloud_testing::assertions::assert_not_empty;

use crate::tests::fixture::tag;

#[actix_rt::test]
async fn add_client_id_to_open_id_connect_provider() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_open_id_provider_output = client
        .create_open_id_connect_provider()
        .url("https://server.example.com")
        .thumbprint_list("c3768084dfb3d2b68b7897bf5f565da8eEXAMPLE")
        .client_id_list("my-application-ID")
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .send()
        .await
        .expect("Failed to create OpenID connect provider");

    client
        .add_client_id_to_open_id_connect_provider()
        .open_id_connect_provider_arn(create_open_id_provider_output.open_id_connect_provider_arn().unwrap())
        .client_id("my-application-ID")
        .send()
        .await
        .expect("Failed to add client ID to OpenID connect provider");

    ctx.stop_server().await;
}
