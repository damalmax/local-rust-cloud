use crate::tests::fixture::{tag, CREATE_USER_PERMISSIONS_BOUNDARY};

#[tokio::test]
async fn untag_mfa_device() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_policy_output = super::fixture::create_policy(
        &client,
        "some-policy-name",
        "some-policy-description",
        "/",
        CREATE_USER_PERMISSIONS_BOUNDARY,
        Some(vec![
            tag("key-1", "value-1"),
            tag("key-2", "value-2"),
            tag("key-3", "value-3"),
            tag("key-4", "value-4"),
            tag("key-5", "value-5"),
        ]),
    )
    .await
    .expect("Failed to create IAM policy");

    let policy_arn = create_policy_output.policy().unwrap().arn().unwrap();

    client
        .untag_policy()
        .policy_arn(policy_arn)
        .tag_keys("key-1")
        .tag_keys("key-2")
        .send()
        .await
        .expect("Failed to untag IAM policy");

    let tags_output = client
        .list_policy_tags()
        .policy_arn(policy_arn)
        .send()
        .await
        .expect("Failed to get a list of tags for IAM policy");

    assert_eq!(tags_output.tags().len(), 3);
}
