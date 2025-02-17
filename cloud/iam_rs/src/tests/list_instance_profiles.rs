use crate::tests::fixture::{tag, CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY};

#[tokio::test]
async fn list_instance_profiles() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    for i in 0..9 {
        client
            .create_instance_profile()
            .path("/")
            .instance_profile_name(format!("instance-profile-{i}"))
            .tags(tag("key1", "value1"))
            .tags(tag("key2", "value2"))
            .send()
            .await
            .unwrap();
    }

    let result = client
        .list_instance_profiles()
        .path_prefix("/")
        .max_items(5)
        .send()
        .await
        .expect("Failed to get a list of IAM instance profiles");

    assert_eq!(result.instance_profiles().len(), 5);
    assert!(result.marker().is_some());
    assert!(result.is_truncated());

    let result = client
        .list_instance_profiles()
        .path_prefix("/")
        .marker(result.marker().unwrap())
        .max_items(5)
        .send()
        .await
        .expect("Failed to get a list of IAM instance profiles");

    assert_eq!(result.instance_profiles().len(), 4);
    assert!(result.marker().is_none());
    assert!(!result.is_truncated());
}

const INSTANCE_PROFILE_NAME: &str = "instance-profile-0";
#[tokio::test]
async fn list_instance_profiles_with_role() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .create_instance_profile()
        .path("/")
        .instance_profile_name(INSTANCE_PROFILE_NAME)
        .send()
        .await
        .unwrap();

    for i in 0..2 {
        let role_name = format!("Test-Role-{i}");
        client
            .create_role()
            .role_name(&role_name)
            .path("/")
            .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
            .send()
            .await
            .expect("Failed to create IAM role");

        client
            .add_role_to_instance_profile()
            .instance_profile_name(INSTANCE_PROFILE_NAME)
            .role_name(&role_name)
            .send()
            .await
            .expect("Failed to add role to instance profile");
    }

    let result = client
        .list_instance_profiles()
        .path_prefix("/")
        .max_items(5)
        .send()
        .await
        .expect("Failed to get a list of IAM instance profiles");

    assert_eq!(result.instance_profiles().len(), 1);
    assert!(result.marker().is_none());
    assert!(!result.is_truncated());

    let roles = result.instance_profiles()[0].roles();
    assert_eq!(roles.len(), 2);
}

#[tokio::test]
async fn list_instance_profiles_empty() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let result = client
        .list_instance_profiles()
        .path_prefix("/")
        .max_items(5)
        .send()
        .await
        .expect("Failed to get a list of IAM instance profiles");

    assert!(result.instance_profiles().is_empty());
}
