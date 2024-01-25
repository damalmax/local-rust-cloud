use crate::tests::fixture::{tag, CREATE_USER_PERMISSIONS_BOUNDARY};
use local_cloud_testing::assertions::assert_not_empty;

#[actix_rt::test]
async fn list_policies_empty() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let result = client
        .list_policies()
        .max_items(10)
        .send()
        .await
        .expect("Failed to get a list of IAM policies");

    assert!(result.policies().is_empty());
    ctx.stop_server().await;
}

#[actix_rt::test]
async fn list_policies_no_marker() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let created_policy = super::fixture::create_policy(
        &client,
        "some-policy-name",
        "some-policy-description",
        "/",
        CREATE_USER_PERMISSIONS_BOUNDARY,
        Some(vec![tag("tag-key1", "tag-value1")]),
    )
    .await
    .unwrap();

    assert!(created_policy.policy().is_some());

    let result = client
        .list_policies()
        .max_items(10)
        .send()
        .await
        .expect("Failed to get a list of IAM policies");

    assert!(!result.policies().is_empty());
    assert_eq!(result.policies().len(), 1);

    let created_policy = created_policy.policy().unwrap();
    let policy = result.policies().get(0).unwrap();
    assert_eq!(policy.policy_name().unwrap(), "some-policy-name");
    assert_not_empty(policy.policy_id());
    assert_eq!(policy.arn().unwrap(), created_policy.arn().unwrap());
    assert_eq!(policy.path().unwrap(), created_policy.path().unwrap());
    assert_eq!(policy.default_version_id().unwrap(), "v1");
    assert_eq!(policy.description().unwrap(), "some-policy-description");
    assert_eq!(policy.attachment_count().unwrap(), 0);
    assert_eq!(policy.permissions_boundary_usage_count().unwrap(), 0);
    assert!(policy.is_attachable());
    assert!(policy.create_date().is_some());
    assert!(policy.update_date().is_some());
    assert_eq!(policy.tags().len(), 1);
    assert_eq!(policy.tags()[0].key(), "tag-key1");
    assert_eq!(policy.tags()[0].value(), "tag-value1");
    ctx.stop_server().await;
}

#[actix_rt::test]
async fn list_policies_with_marker() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let created_policy1 = super::fixture::create_policy(
        &client,
        "some-policy-name",
        "some-policy-description",
        "/",
        CREATE_USER_PERMISSIONS_BOUNDARY,
        Some(vec![tag("tag-key1", "tag-value1")]),
    )
    .await
    .unwrap();

    assert!(created_policy1.policy().is_some());

    let created_policy2 = super::fixture::create_policy(
        &client,
        "some-policy-name2",
        "some-policy-description",
        "/",
        CREATE_USER_PERMISSIONS_BOUNDARY,
        Some(vec![tag("tag-key1", "tag-value1")]),
    )
    .await
    .unwrap();

    assert!(created_policy2.policy().is_some());

    let result = client
        .list_policies()
        .max_items(1)
        .send()
        .await
        .expect("Failed to get a list of IAM policies");

    assert!(!result.policies().is_empty());
    assert_eq!(result.policies().len(), 1);
    assert!(result.is_truncated());
    assert_not_empty(result.marker());

    // requesting second page
    let result = client
        .list_policies()
        .max_items(1)
        .marker(result.marker().unwrap())
        .send()
        .await
        .expect("Failed to get a list of IAM policies");

    assert!(!result.policies().is_empty());
    assert_eq!(result.policies().len(), 1);
    assert!(!result.is_truncated());
    assert!(result.marker().is_none());
    ctx.stop_server().await;
}
