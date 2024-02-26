use crate::tests::fixture::{tag, CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY, CREATE_USER_PERMISSIONS_BOUNDARY};

#[tokio::test]
async fn add_user_to_group() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let policy_output = super::fixture::create_policy(
        &client,
        "test-policy",
        "user-test-policy",
        "/division_abc/subdivision_xyz/",
        CREATE_USER_PERMISSIONS_BOUNDARY,
        None,
    )
    .await
    .unwrap();

    client
        .create_role()
        .role_name("Test-Role")
        .path("/")
        .permissions_boundary(policy_output.policy.unwrap().arn().unwrap())
        .assume_role_policy_document(CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY)
        .tags(tag("create-user-key1", "create-user-value1"))
        .tags(tag("create-user-key2", "create-user-value2"))
        .tags(tag("create-user-key2", "create-user-value3"))
        .send()
        .await
        .expect("Failed to create IAM role");

    client
        .create_instance_profile()
        .path("/")
        .instance_profile_name("instance-profile-1")
        .tags(tag("key1", "value1"))
        .tags(tag("key2", "value2"))
        .send()
        .await
        .expect("Failed to create IAM instance profile");

    let _response = client
        .add_role_to_instance_profile()
        .instance_profile_name("instance-profile-1")
        .role_name("Test-Role")
        .send()
        .await
        .expect("Failed to add role to instance profile");

    ctx.stop_server().await;
}
