use crate::tests::credentials_provider;
use aws_config::BehaviorVersion;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_iam::{config::Region, types::Tag};
use local_cloud_testing::assertions::assert_not_empty;

#[actix_rt::test]
async fn create_policy() {
    let mut ctx = local_cloud_testing::suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = aws_config::SdkConfig::builder()
        .region(Some(Region::new("eu-local-1")))
        .endpoint_url(format!("http://localhost:{}/iam", port))
        .credentials_provider(SharedCredentialsProvider::new(credentials_provider()))
        .behavior_version(BehaviorVersion::latest())
        .build();
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
    let config = aws_config::SdkConfig::builder()
        .region(Some(Region::new("eu-local-1")))
        .endpoint_url(format!("http://localhost:{}/iam", port))
        .credentials_provider(SharedCredentialsProvider::new(credentials_provider()))
        .behavior_version(BehaviorVersion::latest())
        .build();
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
        .set_tags(Option::Some(tags));

    let result = request_builder.send().await;
    assert!(result.is_err());

    let sdk_error = result.unwrap_err();
    assert_eq!(400u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_invalid_input_exception());
    assert_eq!("InvalidInput", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert!(error
        .meta()
        .message()
        .unwrap()
        .eq("The number of submitted tags is larger (51 tags) than allowed (limit: 50 tags)."));

    ctx.stop_server().await;
}
