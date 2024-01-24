use super::fixture;
use aws_sdk_iam::types::Tag;
use local_cloud_testing::assertions::assert_not_empty;

#[actix_rt::test]
async fn test_create_policy() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let response = fixture::create_policy(
        &client,
        "some-policy-name",
        "policy-description",
        "/",
        include_str!("resources/create_user__permissions_boundary.json"),
        Some(vec![
            Tag::builder().key("key1").value("value1").build().unwrap(),
            Tag::builder().key("key2").value("value2").build().unwrap(),
            Tag::builder().key("key3").value("value3").build().unwrap(),
        ]),
    )
    .await
    .unwrap();

    assert!(response.policy().is_some());
    let policy = response.policy().unwrap();
    assert!(policy.tags().len() > 0);
    assert_eq!(policy.default_version_id.as_deref().unwrap(), "v1");
    assert!(policy.create_date.is_some());
    assert_eq!(policy.attachment_count.unwrap(), 0); // new policy is not attached to any user/role/group
    assert_eq!(policy.permissions_boundary_usage_count.unwrap(), 0); // new policy is not attached to any user/role/group
    assert_not_empty(policy.path());
    assert_not_empty(policy.policy_name());
    assert_eq!(policy.is_attachable(), true);

    ctx.stop_server().await;
}

#[actix_rt::test]
async fn create_policy_too_many_tags() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let tags = (0..51)
        .map(|i| {
            Tag::builder()
                .key(format!("key-{}", i))
                .value(format!("value-{}", i))
                .build()
                .unwrap()
        })
        .collect();
    let request_builder = client
        .create_policy()
        .description("policy-description")
        .path("/")
        .policy_document(include_str!("resources/create_user__permissions_boundary.json"))
        .policy_name("some-policy-name")
        .set_tags(Some(tags));

    let result = request_builder.send().await;
    assert!(result.is_err());

    let sdk_error = result.unwrap_err();
    assert_eq!(400u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_invalid_input_exception());
    assert_eq!("InvalidInput", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(error.meta().message().unwrap(), "Number of '$.Tags' cannot be greater than 50.");

    ctx.stop_server().await;
}
