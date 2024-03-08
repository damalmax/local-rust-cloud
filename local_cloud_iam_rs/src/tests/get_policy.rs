use local_cloud_testing::assertions::assert_not_empty;

use crate::tests::fixture::{tag, CREATE_USER_PERMISSIONS_BOUNDARY};

use super::fixture;

#[tokio::test]
async fn get_policy() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_policy_output = fixture::create_policy(
        &client,
        "some-policy-name",
        "policy-description",
        "/",
        CREATE_USER_PERMISSIONS_BOUNDARY,
        Some(vec![tag("key1", "value1"), tag("key2", "value2"), tag("key3", "value3")]),
    )
    .await
    .unwrap();

    let arn = create_policy_output.policy().unwrap().arn().unwrap();

    let response = client
        .get_policy()
        .policy_arn(arn)
        .send()
        .await
        .expect("Failed to get IAM policy");

    assert!(response.policy().is_some());
    let policy = response.policy().unwrap();
    assert!(policy.tags().is_empty());
    assert_eq!(policy.default_version_id.as_deref().unwrap(), "v1");
    assert!(policy.create_date.is_some());
    assert_eq!(policy.attachment_count.unwrap(), 0); // new policy is not attached to any user/role/group
    assert_eq!(policy.permissions_boundary_usage_count.unwrap(), 0); // new policy is not attached to any user/role/group
    assert_not_empty(policy.path());
    assert_not_empty(policy.policy_name());
    assert!(policy.is_attachable());
}

#[tokio::test]
async fn get_policy_does_not_exist() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let arn = "arn:aws:iam::000000000001:policy/unknown";

    let response = client.get_policy().policy_arn(arn).send().await;
    assert!(response.is_err());
    let sdk_error = response.unwrap_err();
    assert_eq!(404u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_no_such_entity_exception());
    assert_eq!("NoSuchEntity", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(
        error.meta().message().unwrap(),
        "IAM policy with ARN 'arn:aws:iam::000000000001:policy/unknown' doesn't exist."
    );
}
