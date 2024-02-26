use crate::tests::fixture::{tag, CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY};

#[tokio::test]
async fn test_tag_role() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_role_output = client
        .create_role()
        .role_name("Test-Role")
        .path("/")
        .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
        .tags(tag("create-user-key1", "create-user-value1"))
        .tags(tag("create-user-key2", "create-user-value2"))
        .tags(tag("create-user-key2", "create-user-value3"))
        .send()
        .await
        .expect("Failed to create IAM role");

    client
        .tag_role()
        .role_name(&create_role_output.role().unwrap().role_name)
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .tags(tag("key3", "value3"))
        .tags(tag("key4", "value4"))
        .tags(tag("key5", "value5"))
        .send()
        .await
        .expect("Failed to tag IAM role");

    ctx.stop_server().await;
}

#[tokio::test]
async fn test_tag_role_limit_exceeded() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_role_output = client
        .create_role()
        .role_name("Test-Role")
        .path("/")
        .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
        .tags(tag("key-1", "create-user-value1"))
        .tags(tag("key-2", "create-user-value2"))
        .tags(tag("key-3", "create-user-value3"))
        .tags(tag("key-4", "create-user-value3"))
        .tags(tag("key-5", "create-user-value3"))
        .send()
        .await
        .expect("Failed to create IAM role");

    let tags = (5..=51)
        .map(|i| tag(format!("key-{}", i).as_str(), format!("value-{}", i).as_str()))
        .collect();
    let result = client
        .tag_role()
        .role_name(&create_role_output.role().unwrap().role_name)
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
    assert_eq!(error.meta().message().unwrap(), "Cannot assign more than 50 tags to IAM role.");

    ctx.stop_server().await;
}

#[tokio::test]
async fn test_tag_role_with_replacement() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_role_output = client
        .create_role()
        .role_name("Test-Role")
        .path("/")
        .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
        .tags(tag("key-1", "create-user-value1"))
        .tags(tag("key-2", "create-user-value2"))
        .tags(tag("key-3", "create-user-value3"))
        .tags(tag("key-4", "create-user-value3"))
        .tags(tag("key-5", "create-user-value3"))
        .send()
        .await
        .expect("Failed to create IAM role");

    let tags = (1..=50)
        .map(|i| tag(format!("key-{}", i).as_str(), format!("value-{}", i).as_str()))
        .collect();
    client
        .tag_role()
        .role_name(&create_role_output.role().unwrap().role_name)
        .set_tags(Some(tags))
        .send()
        .await
        .expect("Failed to assign maximum allowed number of tags with value replacements to IAM role");

    ctx.stop_server().await;
}
