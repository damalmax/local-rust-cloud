use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;

#[tokio::test]
async fn set_default_policy_version() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
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
        .set_as_default(true)
        .send()
        .await
        .expect("Failed to create IAM policy version");

    let get_policy_output = client
        .get_policy()
        .policy_arn(policy_arn)
        .send()
        .await
        .expect("Failed to get IAM policy");

    assert_eq!(get_policy_output.policy().unwrap().default_version_id().unwrap(), "v2");

    client
        .set_default_policy_version()
        .policy_arn(policy_arn)
        .version_id("v1")
        .send()
        .await
        .expect("Failed to change default policy version");

    let get_policy_output = client
        .get_policy()
        .policy_arn(policy_arn)
        .send()
        .await
        .expect("Failed to get IAM policy");

    assert_eq!(get_policy_output.policy().unwrap().default_version_id().unwrap(), "v1");

    ctx.stop_server().await;
}

#[tokio::test]
async fn set_default_policy_version_version_does_not_exist() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
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

    let get_policy_output = client
        .get_policy()
        .policy_arn(policy_arn)
        .send()
        .await
        .expect("Failed to get IAM policy");

    assert_eq!(get_policy_output.policy().unwrap().default_version_id().unwrap(), "v1");

    let response = client
        .set_default_policy_version()
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

    ctx.stop_server().await;
}
