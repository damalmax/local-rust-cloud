use testing::assertions::assert_not_empty;

use crate::tests::fixture::tag;

#[tokio::test]
async fn list_groups_for_user_with_marker() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    let user_name = "user1";

    super::fixture::create_user(
        &client,
        user_name,
        "/",
        None,
        Some(vec![
            tag("create-user-key1", "create user value 1"),
            tag("create-user-key2", "create user value 2"),
            tag("create-user-key3", "create user value 3"),
        ]),
    )
    .await
    .expect("Failed to create IAM user");

    for i in 0..10 {
        client
            .create_group()
            .group_name(format!("test_group_{i}"))
            .path("/")
            .send()
            .await
            .expect("Failed to create IAM group");
    }

    for i in 0..10 {
        client
            .add_user_to_group()
            .user_name(user_name)
            .group_name(format!("test_group_{i}"))
            .send()
            .await
            .expect("Failed to add user to group");
    }

    let result = client
        .list_groups_for_user()
        .user_name(user_name)
        .max_items(5)
        .send()
        .await
        .expect("Failed to list groups for user");

    assert_eq!(result.groups().len(), 5);
    assert!(result.is_truncated());
    assert!(result.marker().is_some());
    assert_not_empty(result.marker());

    let result = client
        .list_groups_for_user()
        .user_name(user_name)
        .max_items(5)
        .marker(result.marker().unwrap())
        .send()
        .await
        .expect("Failed to get second page of list groups for user result");

    assert_eq!(result.groups().len(), 5);
    assert!(!result.is_truncated());
    assert!(result.marker().is_none());
}
