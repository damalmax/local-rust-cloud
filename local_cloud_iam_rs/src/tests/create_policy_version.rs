use crate::tests::fixture::{tag, CREATE_USER_PERMISSIONS_BOUNDARY};

#[tokio::test]
async fn create_policy_version() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let response = client
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

    let create_policy_version_response = client
        .create_policy_version()
        .policy_arn(response.policy.unwrap().arn.unwrap())
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .set_as_default(false)
        .send()
        .await
        .expect("Failed to create IAM policy version");

    assert!(create_policy_version_response.policy_version().is_some());

    ctx.stop_server().await;
}

#[tokio::test]
async fn create_policy_version_limit_exceeded() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    // create policy (first policy version will be created with this policy)
    let response = client
        .create_policy()
        .description("policy-description")
        .path("/")
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .policy_name("some-policy-name2")
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .send()
        .await
        .expect("Failed to create IAM policy");

    let policy_arn = response.policy.unwrap().arn.unwrap();

    // create 4 other policy versions
    for _i in 0..4 {
        client
            .create_policy_version()
            .policy_arn(&policy_arn)
            .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
            .set_as_default(true)
            .send()
            .await
            .expect("Failed to create IAM policy version");
    }

    let result = client
        .create_policy_version()
        .policy_arn(&policy_arn)
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .set_as_default(true)
        .send()
        .await;

    assert!(result.is_err());
    let sdk_error = result.unwrap_err();
    assert_eq!(409u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_limit_exceeded_exception());
    assert_eq!("LimitExceeded", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(
        error.meta().message().unwrap(),
        "Number of Policy Versions cannot be greater than '5'. Actual count: '5'."
    );

    ctx.stop_server().await;
}
