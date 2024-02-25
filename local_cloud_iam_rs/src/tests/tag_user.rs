use crate::tests::fixture::tag;

#[tokio::test]
async fn test_tag_user() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_user_output = super::fixture::create_user(
        &client,
        "user1",
        "/",
        None,
        Some(vec![
            tag("key-1", "create-user-value1"),
            tag("key-2", "create-user-value2"),
            tag("key-3", "create-user-value3"),
        ]),
    )
    .await
    .expect("Failed to create IAM user");

    client
        .tag_user()
        .user_name(create_user_output.user().unwrap().user_name())
        .tags(tag("key-1", "value1"))
        .tags(tag("key-2", "value2"))
        .tags(tag("key-3", "value3"))
        .tags(tag("key-4", "value4"))
        .tags(tag("key-5", "value5"))
        .send()
        .await
        .expect("Failed to tag IAM policy");

    ctx.stop_server().await;
}

#[tokio::test]
async fn test_tag_policy_limit_exceeded() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_user_output = super::fixture::create_user(
        &client,
        "user1",
        "/",
        None,
        Some(vec![
            tag("key-1", "create-user-value1"),
            tag("key-2", "create-user-value2"),
            tag("key-3", "create-user-value3"),
            tag("key-4", "create-user-value3"),
            tag("key-5", "create-user-value3"),
        ]),
    )
    .await
    .expect("Failed to create IAM user");

    let tags = (5..=51)
        .map(|i| tag(format!("key-{}", i).as_str(), format!("value-{}", i).as_str()))
        .collect();
    let result = client
        .tag_user()
        .user_name(create_user_output.user().unwrap().user_name())
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
    assert_eq!(error.meta().message().unwrap(), "Cannot assign more than 50 tags to IAM user.");

    ctx.stop_server().await;
}

#[tokio::test]
async fn test_tag_user_with_replacement() {
    let mut ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_user_output = super::fixture::create_user(
        &client,
        "user1",
        "/",
        None,
        Some(vec![
            tag("key-1", "create-user-value1"),
            tag("key-2", "create-user-value2"),
            tag("key-3", "create-user-value3"),
            tag("key-4", "create-user-value3"),
            tag("key-5", "create-user-value3"),
        ]),
    )
    .await
    .expect("Failed to create IAM user");

    let tags = (1..=50)
        .map(|i| tag(format!("key-{}", i).as_str(), format!("value-{}", i).as_str()))
        .collect();
    client
        .tag_user()
        .user_name(create_user_output.user().unwrap().user_name())
        .set_tags(Some(tags))
        .send()
        .await
        .expect("Failed to assign maximum allowed number of tags with value replacements to IAM user");

    ctx.stop_server().await;
}
