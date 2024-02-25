use crate::tests::fixture::{tag, CREATE_USER_PERMISSIONS_BOUNDARY};

#[tokio::test]
async fn list_policy_versions_with_marker() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_policy_output = client
        .create_policy()
        .description("policy-description")
        .path("/")
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .policy_name("some-policy-name")
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .tags(tag("key3", "value3"))
        .send()
        .await
        .expect("Failed to create IAM policy");

    let arn = create_policy_output.policy().unwrap().arn().unwrap();

    for _i in 0..=3 {
        client
            .create_policy_version()
            .policy_arn(arn)
            .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
            .set_as_default(false)
            .send()
            .await
            .expect("Failed to create IAM policy version");
    }

    let result = client
        .list_policy_versions()
        .max_items(3)
        .policy_arn(arn)
        .send()
        .await
        .expect("Failed to get a list of IAM policy versions");

    assert_eq!(result.versions().len(), 3);
    assert!(result.marker().is_some());
    assert!(result.is_truncated());

    let result = client
        .list_policy_versions()
        .max_items(3)
        .policy_arn(arn)
        .marker(result.marker().unwrap())
        .send()
        .await
        .expect("Failed to get a second page of a list of IAM policy versions");

    assert_eq!(result.versions().len(), 2);
    assert!(result.marker().is_none());
    assert!(!result.is_truncated());

    ctx.stop_server().await;
}
