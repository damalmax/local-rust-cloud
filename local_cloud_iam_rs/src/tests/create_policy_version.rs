use aws_sdk_iam::types::Tag;

#[actix_rt::test]
async fn create_policy_version() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let response = client
        .create_policy()
        .description("policy-description")
        .path("/")
        .policy_document(include_str!("resources/create_user__permissions_boundary.json"))
        .policy_name("some-policy-name")
        .tags(Tag::builder().key("key1").value("value1").build().unwrap())
        .tags(Tag::builder().key("key2").value("value2").build().unwrap())
        .tags(Tag::builder().key("key3").value("value3").build().unwrap())
        .send()
        .await
        .expect("Failed to create IAM policy");

    let create_policy_version_response = client
        .create_policy_version()
        .policy_arn(response.policy.unwrap().arn.unwrap())
        .policy_document(include_str!("resources/create_user__permissions_boundary.json"))
        .set_as_default(false)
        .send()
        .await
        .expect("Failed to create IAM policy version");

    assert!(create_policy_version_response.policy_version().is_some());

    ctx.stop_server().await;
}

#[actix_rt::test]
async fn create_policy_version_limit_exceeded() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    // create policy (first policy version will be created with this policy)
    let response = client
        .create_policy()
        .description("policy-description")
        .path("/")
        .policy_document(include_str!("resources/create_user__permissions_boundary.json"))
        .policy_name("some-policy-name2")
        .tags(Tag::builder().key("key1").value("value1").build().unwrap())
        .tags(Tag::builder().key("key2").value("value2").build().unwrap())
        .send()
        .await
        .expect("Failed to create IAM policy");

    let policy_arn = response.policy.unwrap().arn.unwrap();

    // create 4 other policy versions
    for i in 0..4 {
        client
            .create_policy_version()
            .policy_arn(&policy_arn)
            .policy_document(include_str!("resources/create_user__permissions_boundary.json"))
            .set_as_default(true)
            .send()
            .await
            .expect("Failed to create IAM policy version");
    }

    let result = client
        .create_policy_version()
        .policy_arn(&policy_arn)
        .policy_document(include_str!("resources/create_user__permissions_boundary.json"))
        .set_as_default(true)
        .send()
        .await;

    assert!(result.is_err());
    let sdk_error = result.unwrap_err();
    assert_eq!(400u16, sdk_error.raw_response().unwrap().status().as_u16());

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
