use testing::assertions::assert_not_empty;

use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;

#[tokio::test]
async fn get_policy_version() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let response = client
        .create_policy()
        .description("policy-description")
        .path("/")
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .policy_name("some-policy-name")
        .send()
        .await
        .expect("Failed to create IAM policy");

    let policy_arn = response.policy().unwrap().arn().unwrap();
    client
        .create_policy_version()
        .policy_arn(policy_arn)
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .set_as_default(false)
        .send()
        .await
        .expect("Failed to create IAM policy version");

    let response = client
        .get_policy_version()
        .policy_arn(policy_arn)
        .version_id("v2")
        .send()
        .await
        .expect("Failed to get IAM policy version");

    let policy_version = response.policy_version().unwrap();
    assert_eq!(policy_version.version_id().unwrap(), "v2");
    assert!(!policy_version.is_default_version());
    assert!(policy_version.create_date().is_some());
    assert_not_empty(policy_version.document());
}

#[tokio::test]
async fn get_policy_version_does_not_exist() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let response = client
        .create_policy()
        .description("policy-description")
        .path("/")
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .policy_name("some-policy-name")
        .send()
        .await
        .expect("Failed to create IAM policy");

    let policy_arn = response.policy().unwrap().arn().unwrap();

    let response = client
        .get_policy_version()
        .policy_arn(policy_arn)
        .version_id("v2")
        .send()
        .await;

    assert!(response.is_err());
    let sdk_error = response.unwrap_err();
    assert_eq!(404u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_no_such_entity_exception());
    assert_eq!("NoSuchEntity", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(error.meta().message().unwrap(), "Entity does not exist.");
}
