use crate::tests::fixture::tag;

#[tokio::test]
async fn tag_instance_profile() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_instance_profile_output = client
        .create_instance_profile()
        .path("/")
        .instance_profile_name("instance-profile-1")
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .send()
        .await
        .expect("Failed to create IAM instance profile");

    client
        .tag_instance_profile()
        .instance_profile_name(
            create_instance_profile_output
                .instance_profile()
                .unwrap()
                .instance_profile_name(),
        )
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .tags(tag("key3", "value3"))
        .tags(tag("key4", "value4"))
        .tags(tag("key5", "value5"))
        .send()
        .await
        .expect("Failed to tag IAM instance profile");
}

#[tokio::test]
async fn tag_instance_profile_limit_exceeded() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_instance_profile_output = client
        .create_instance_profile()
        .path("/")
        .instance_profile_name("instance-profile-1")
        .tags(tag("key-1", "value1"))
        .tags(tag("key-2", "value2"))
        .tags(tag("key-3", "value3"))
        .tags(tag("key-4", "value4"))
        .tags(tag("key-5", "value5"))
        .send()
        .await
        .expect("Failed to create IAM instance profile");

    let tags = (5..=51)
        .map(|i| tag(format!("key-{}", i).as_str(), format!("value-{}", i).as_str()))
        .collect();
    let result = client
        .tag_instance_profile()
        .instance_profile_name(
            create_instance_profile_output
                .instance_profile()
                .unwrap()
                .instance_profile_name(),
        )
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
    assert_eq!(error.meta().message().unwrap(), "Cannot assign more than 50 tags to IAM instance profile.");
}

#[tokio::test]
async fn tag_instance_profile_with_replacement() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let create_instance_profile_output = client
        .create_instance_profile()
        .path("/")
        .instance_profile_name("instance-profile-1")
        .tags(tag("key-1", "value1"))
        .tags(tag("key-2", "value2"))
        .tags(tag("key-3", "value3"))
        .tags(tag("key-4", "value4"))
        .tags(tag("key-5", "value5"))
        .send()
        .await
        .expect("Failed to create IAM instance profile");

    let tags = (1..=50)
        .map(|i| tag(format!("key-{}", i).as_str(), format!("value-{}", i).as_str()))
        .collect();
    client
        .tag_instance_profile()
        .instance_profile_name(
            create_instance_profile_output
                .instance_profile()
                .unwrap()
                .instance_profile_name(),
        )
        .set_tags(Some(tags))
        .send()
        .await
        .expect("Failed to assign maximum allowed number of tags with value replacements to IAM instance profile");
}
