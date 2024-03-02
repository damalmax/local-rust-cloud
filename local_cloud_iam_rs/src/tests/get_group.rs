use crate::tests::fixture::{tag, CREATE_USER_PERMISSIONS_BOUNDARY};
use local_cloud_testing::assertions::assert_not_empty;

#[tokio::test]
async fn get_group_no_users() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    client
        .create_group()
        .group_name("test_group_1")
        .path("/")
        .send()
        .await
        .expect("Failed to create IAM group");

    let response = client
        .get_group()
        .group_name("test_group_1")
        .send()
        .await
        .expect("Failed to execute IAM get_group");

    assert!(response.group().is_some());
    let group = response.group().unwrap();
    assert_not_empty(group.path());
    assert_not_empty(group.arn());
    assert_not_empty(group.group_id());
    assert_not_empty(group.group_name());

    assert_eq!(response.users.len(), 0);
    assert!(!response.is_truncated);
    assert!(response.marker.is_none());
    ctx.stop_server().await;
}

#[tokio::test]
async fn get_group_group_does_not_exist() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let response = client.get_group().group_name("test_group_1").send().await;

    assert!(response.is_err());
    let sdk_error = response.unwrap_err();
    assert_eq!(404u16, sdk_error.raw_response().unwrap().status().as_u16());

    let error = sdk_error.into_service_error();
    assert!(error.is_no_such_entity_exception());
    assert_eq!("NoSuchEntity", error.meta().code().unwrap());
    assert!(error.meta().message().unwrap().len() > 0);
    assert_eq!(error.meta().message().unwrap(), "IAM group with name 'test_group_1' doesn't exist.");

    ctx.stop_server().await;
}

#[tokio::test]
async fn get_group_with_users() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let _create_group_output = client
        .create_group()
        .group_name("test_group_1")
        .path("/")
        .send()
        .await
        .expect("Failed to create IAM group");

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

    let _create_user_output = super::fixture::create_user(
        &client,
        "user1",
        "/",
        policy_output.policy.unwrap().arn(),
        Some(vec![
            tag("create-user-key1", "create user value1"),
            tag("create-user-key2", "create user value2"),
            tag("create-user-key3", "create user value3"),
        ]),
    )
    .await
    .expect("Failed to create IAM user");

    let _add_user_to_group_output = client
        .add_user_to_group()
        .user_name("user1")
        .group_name("test_group_1")
        .send()
        .await
        .expect("Failed to add user to group");

    let response = client
        .get_group()
        .group_name("test_group_1")
        .send()
        .await
        .expect("Failed to execute IAM get_group");

    assert!(response.group().is_some());
    let group = response.group().unwrap();
    assert_not_empty(group.path());
    assert_not_empty(group.arn());
    assert_not_empty(group.group_id());
    assert_not_empty(group.group_name());

    assert_eq!(response.users.len(), 1);
    let user = &response.users[0];
    assert_eq!(user.tags().len(), 0);
    assert_not_empty(user.path());
    assert_not_empty(user.arn());
    assert_not_empty(user.user_name());
    assert_not_empty(user.user_id());
    assert!(user.permissions_boundary.is_none());

    assert!(!response.is_truncated);
    assert!(response.marker.is_none());
    ctx.stop_server().await;
}
