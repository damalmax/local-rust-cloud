use crate::tests::fixture::{tag, CREATE_USER_PERMISSIONS_BOUNDARY};

use super::fixture;

#[actix_rt::test]
async fn test_tag_policy() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_policy_output = fixture::create_policy(
        &client,
        "some-policy-name",
        "policy-description",
        "/",
        CREATE_USER_PERMISSIONS_BOUNDARY,
        None,
    )
    .await
    .expect("Failed to create IAM policy");

    client
        .tag_policy()
        .policy_arn(create_policy_output.policy().unwrap().arn().unwrap())
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .tags(tag("key3", "value3"))
        .tags(tag("key4", "value4"))
        .tags(tag("key5", "value5"))
        .send()
        .await
        .expect("Failed to tag IAM policy");

    ctx.stop_server().await;
}

#[actix_rt::test]
async fn test_tag_policy_limit_exceeded() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    // Create Policy with 5 tags attached
    let create_policy_output = fixture::create_policy(
        &client,
        "some-policy-name",
        "policy-description",
        "/",
        CREATE_USER_PERMISSIONS_BOUNDARY,
        Some(vec![
            tag("key-1", "value1"),
            tag("key-2", "value2"),
            tag("key-3", "value3"),
            tag("key-4", "value4"),
            tag("key-5", "value5"),
        ]),
    )
    .await
    .expect("Failed to create IAM policy");

    let tags = (5..=51)
        .map(|i| tag(format!("key-{}", i).as_str(), format!("value-{}", i).as_str()))
        .collect();
    let result = client
        .tag_policy()
        .policy_arn(create_policy_output.policy().unwrap().arn().unwrap())
        .set_tags(Some(tags))
        .send()
        .await;

    assert!(result.is_err());
    let sdk_error = result.unwrap_err();
    assert_eq!(409u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_limit_exceeded_exception());
    assert_eq!("LimitExceeded", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(error.meta().message().unwrap(), "Cannot assign more than 50 tags to IAM policy.");

    ctx.stop_server().await;
}

#[actix_rt::test]
async fn test_tag_policy_with_replacement() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    // Create Policy with 5 tags attached
    let create_policy_output = fixture::create_policy(
        &client,
        "some-policy-name",
        "policy-description",
        "/",
        CREATE_USER_PERMISSIONS_BOUNDARY,
        Some(vec![
            tag("key-1", "value1"),
            tag("key-2", "value2"),
            tag("key-3", "value3"),
            tag("key-4", "value4"),
            tag("key-5", "value5"),
        ]),
    )
    .await
    .expect("Failed to create IAM policy");

    let tags = (1..=50)
        .map(|i| tag(format!("key-{}", i).as_str(), format!("value-{}", i).as_str()))
        .collect();
    client
        .tag_policy()
        .policy_arn(create_policy_output.policy().unwrap().arn().unwrap())
        .set_tags(Some(tags))
        .send()
        .await
        .expect("Failed to assign maximum allowed number of tags with value replacements to IAM policy");

    ctx.stop_server().await;
}
