use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;
use local_cloud_testing::assertions::assert_not_empty;

#[tokio::test]
async fn list_group_policies_with_marker() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let _create_group_output = client
        .create_group()
        .group_name("test_group_1")
        .path("/")
        .send()
        .await
        .expect("Failed to create IAM group");

    for i in 0..10 {
        client
            .put_group_policy()
            .group_name("test_group_1")
            .policy_name(format!("test_policy_{i}"))
            .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
            .send()
            .await
            .expect("Failed to put IAM group policy");
    }

    let result = client
        .list_group_policies()
        .max_items(5)
        .group_name("test_group_1")
        .send()
        .await
        .expect("Failed to get a list of group policies");

    assert!(result.is_truncated);
    assert_eq!(result.policy_names().len(), 5);
    assert_not_empty(result.marker());

    let result = client
        .list_group_policies()
        .max_items(5)
        .group_name("test_group_1")
        .marker(result.marker().unwrap())
        .send()
        .await
        .expect("Failed to get second page of a list of group policies");

    assert!(!result.is_truncated);
    assert_eq!(result.policy_names().len(), 5);
    assert!(result.marker().is_none());

    ctx.stop_server().await;
}
