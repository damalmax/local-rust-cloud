use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;

#[tokio::test]
async fn delete_policy_version() {
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

    let create_policy_version_response = client
        .create_policy_version()
        .policy_arn(policy_arn)
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .set_as_default(false)
        .send()
        .await
        .expect("Failed to create IAM policy version");

    client
        .delete_policy_version()
        .policy_arn(policy_arn)
        .version_id(
            create_policy_version_response
                .policy_version()
                .unwrap()
                .version_id()
                .unwrap(),
        )
        .send()
        .await
        .expect("Failed to delete IAM policy version");
}

#[tokio::test]
async fn delete_policy_version_delete_default() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let response = client
        .create_policy()
        .description("policy-description")
        .path("/")
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .policy_name("some-policy-name2")
        .send()
        .await
        .expect("Failed to create IAM policy");

    let policy_arn = response.policy().unwrap().arn().unwrap();

    let create_policy_version_response = client
        .create_policy_version()
        .policy_arn(policy_arn)
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .set_as_default(true)
        .send()
        .await
        .expect("Failed to create IAM policy version");

    let version_id = create_policy_version_response
        .policy_version()
        .unwrap()
        .version_id()
        .unwrap();

    let result = client
        .delete_policy_version()
        .policy_arn(policy_arn)
        .version_id(version_id)
        .send()
        .await;

    assert!(result.is_err());
    let sdk_error = result.unwrap_err();
    assert_eq!(409u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_delete_conflict_exception());
    assert_eq!("DeleteConflict", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(error.meta().message().unwrap(), "The policy version is set as default.");
}

#[tokio::test]
async fn delete_policy_version_policy_version_does_not_exist() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let response = client
        .create_policy()
        .description("policy-description")
        .path("/")
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .policy_name("some-policy-name2")
        .send()
        .await
        .expect("Failed to create IAM policy");

    let policy_arn = response.policy().unwrap().arn().unwrap();

    let result = client
        .delete_policy_version()
        .policy_arn(policy_arn)
        .version_id("v2")
        .send()
        .await;

    assert!(result.is_err());
    let sdk_error = result.unwrap_err();
    assert_eq!(404u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_no_such_entity_exception());
    assert_eq!("NoSuchEntity", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(error.meta().message().unwrap(), "Entity does not exist.");
}
