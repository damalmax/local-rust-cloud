use local_cloud_testing::assertions::assert_not_empty;

use crate::tests::fixture::tag;

#[tokio::test]
async fn create_open_id_connect_provider() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let response = client
        .create_open_id_connect_provider()
        .url("https://server.example.com")
        .thumbprint_list("c3768084dfb3d2b68b7897bf5f565da8eEXAMPLE")
        .client_id_list("my-application-ID")
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .send()
        .await
        .unwrap();

    assert_not_empty(response.open_id_connect_provider_arn());
    assert_eq!(
        response.open_id_connect_provider_arn().unwrap(),
        "arn:aws:iam::000000000001:oidc-provider/server.example.com"
    );
    assert_eq!(response.tags().len(), 2);

    ctx.stop_server().await;
}
