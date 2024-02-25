use crate::tests::fixture::{tag, CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY};

const ROLE_NAME: &str = "Test-Role-1";

#[tokio::test]
async fn untag_role() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_role_output = client
        .create_role()
        .role_name(ROLE_NAME)
        .path("/")
        .set_permissions_boundary(None)
        .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
        .tags(tag("key-1", "value-1"))
        .tags(tag("key-2", "value-2"))
        .tags(tag("key-3", "value-3"))
        .tags(tag("key-4", "value-4"))
        .tags(tag("key-5", "value-5"))
        .send()
        .await
        .expect("Failed to create IAM role");

    client
        .untag_role()
        .role_name(ROLE_NAME)
        .tag_keys("key-1")
        .tag_keys("key-2")
        .send()
        .await
        .expect("Failed to untag IAM role");

    let tags_output = client
        .list_role_tags()
        .role_name(ROLE_NAME)
        .send()
        .await
        .expect("Failed to get a list of tags for IAM role");

    assert_eq!(tags_output.tags().len(), 3);

    ctx.stop_server().await;
}
