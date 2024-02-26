use local_cloud_testing::assertions::assert_not_empty;

use crate::tests::fixture::tag;

#[tokio::test]
async fn list_open_id_connect_provider_tags() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_open_id_connect_provider_output = client
        .create_open_id_connect_provider()
        .url("https://server.example.com")
        .thumbprint_list("c3768084dfb3d2b68b7897bf5f565da8eEXAMPLE")
        .client_id_list("my-application-ID")
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .tags(tag("key3", "value3"))
        .tags(tag("key4", "value4"))
        .tags(tag("key5", "value5"))
        .send()
        .await
        .unwrap();

    let arn = create_open_id_connect_provider_output
        .open_id_connect_provider_arn()
        .unwrap();

    let result = client
        .list_open_id_connect_provider_tags()
        .open_id_connect_provider_arn(arn)
        .max_items(3)
        .send()
        .await
        .expect("Failed to get a list of IAM OpenID connect provider tags");

    assert!(!result.tags().is_empty());
    assert_eq!(result.tags().len(), 3);
    assert!(result.is_truncated());
    assert_not_empty(result.marker());

    // requesting second page
    let result = client
        .list_open_id_connect_provider_tags()
        .open_id_connect_provider_arn(arn)
        .marker(result.marker().unwrap())
        .send()
        .await
        .expect("Failed to get a list of IAM OpenID connect provider tags");

    assert!(!result.tags().is_empty());
    assert_eq!(result.tags().len(), 2);
    assert!(!result.is_truncated());
    assert!(result.marker().is_none());

    ctx.stop_server().await;
}
