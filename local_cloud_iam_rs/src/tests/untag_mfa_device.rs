use crate::tests::fixture::tag;

#[actix_rt::test]
async fn untag_mfa_device() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
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

    client
        .untag_mfa_device()
        .serial_number(serial_number)
        .tag_keys("key-1")
        .tag_keys("key-2")
        .send()
        .await
        .expect("Failed to untag IAM MFA device");

    let tags_output = client
        .list_mfa_device_tags()
        .serial_number(serial_number)
        .send()
        .await
        .expect("Failed to get a list of tags for IAM MFA device");

    assert_eq!(tags_output.tags().len(), 3);

    ctx.stop_server().await;
}
