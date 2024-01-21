use aws_sdk_iam::types::Tag;

#[actix_rt::test]
async fn create_user() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let response = client
        .create_user()
        .user_name("user1")
        .path("/")
        .permissions_boundary("")
        .tags(Tag::builder().key("key1").value("value1").build().unwrap())
        .tags(Tag::builder().key("key2").value("value2").build().unwrap())
        .tags(Tag::builder().key("key2").value("value3").build().unwrap())
        .send()
        .await
        .expect("Failed to create IAM policy");

    assert!(response.user().is_some());

    ctx.stop_server().await;
}
