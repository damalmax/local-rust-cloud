use local_cloud_testing::assertions::assert_not_empty;

use crate::tests::fixture::tag;

#[tokio::test]
async fn list_mfa_device_tags_with_marker() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let mfa_device_output = client
        .create_virtual_mfa_device()
        .virtual_mfa_device_name("Device1")
        .tags(tag("key-1", "value-1"))
        .tags(tag("key-2", "value-2"))
        .tags(tag("key-3", "value-3"))
        .tags(tag("key-4", "value-4"))
        .tags(tag("key-5", "value-5"))
        .send()
        .await
        .expect("Failed to create Virtual MFA device");

    let serial_number = mfa_device_output.virtual_mfa_device().unwrap().serial_number();

    let result = client
        .list_mfa_device_tags()
        .serial_number(serial_number)
        .max_items(3)
        .send()
        .await
        .expect("Failed to get a list of IAM MFA device tags");

    assert!(!result.tags().is_empty());
    assert_eq!(result.tags().len(), 3);
    assert!(result.is_truncated());
    assert_not_empty(result.marker());

    // requesting second page
    let result = client
        .list_mfa_device_tags()
        .serial_number(serial_number)
        .marker(result.marker().unwrap())
        .send()
        .await
        .expect("Failed to get a list of IAM MFA device tags");

    assert!(!result.tags().is_empty());
    assert_eq!(result.tags().len(), 2);
    assert!(!result.is_truncated());
    assert!(result.marker().is_none());
    ctx.stop_server().await;
}
