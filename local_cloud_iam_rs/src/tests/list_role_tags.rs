use local_cloud_testing::assertions::assert_not_empty;

use crate::tests::fixture::{tag, CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY};

#[tokio::test]
async fn list_role_tags_with_marker() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_role_output = client
        .create_role()
        .role_name("Test-Role")
        .path("/")
        .set_permissions_boundary(None)
        .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
        .tags(tag("create-role-key1", "create-role-value1"))
        .tags(tag("create-role-key2", "create-role-value2"))
        .tags(tag("create-role-key3", "create-role-value3"))
        .tags(tag("create-role-key4", "create-role-value4"))
        .tags(tag("create-role-key5", "create-role-value5"))
        .tags(tag("create-role-key6", "create-role-value6"))
        .send()
        .await
        .expect("Failed to create IAM role");

    assert!(create_role_output.role().is_some());

    let result = client
        .list_role_tags()
        .role_name("Test-Role")
        .max_items(3)
        .send()
        .await
        .expect("Failed to get a list of IAM policy tags");

    assert!(!result.tags().is_empty());
    assert_eq!(result.tags().len(), 3);
    assert!(result.is_truncated());
    assert_not_empty(result.marker());

    // requesting second page
    let result = client
        .list_role_tags()
        .role_name("Test-Role")
        .marker(result.marker().unwrap())
        .send()
        .await
        .expect("Failed to get a list of IAM policy tags");

    assert!(!result.tags().is_empty());
    assert_eq!(result.tags().len(), 3);
    assert!(!result.is_truncated());
    assert!(result.marker().is_none());
    ctx.stop_server().await;
}
