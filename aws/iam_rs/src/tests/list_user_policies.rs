use testing::assertions::assert_not_empty;

use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;

#[tokio::test]
async fn list_role_policies_with_marker() {
    let ctx = testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let port = ctx.port;
    let config = super::aws_config(port);
    let client = aws_sdk_iam::Client::new(&config);

    super::fixture::create_user(&client, "user-1", "/", None, None)
        .await
        .expect("Failed to create IAM user");

    for i in 0..10 {
        client
            .put_user_policy()
            .user_name("user-1")
            .policy_name(format!("test_policy_{i}"))
            .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
            .send()
            .await
            .expect("Failed to put IAM user policy");
    }

    let result = client
        .list_user_policies()
        .max_items(5)
        .user_name("user-1")
        .send()
        .await
        .expect("Failed to get a list of user policies");

    assert!(result.is_truncated);
    assert_eq!(result.policy_names().len(), 5);
    assert_not_empty(result.marker());

    let result = client
        .list_user_policies()
        .max_items(5)
        .user_name("user-1")
        .marker(result.marker().unwrap())
        .send()
        .await
        .expect("Failed to get second page of a list of user policies");

    assert!(!result.is_truncated);
    assert_eq!(result.policy_names().len(), 5);
    assert!(result.marker().is_none());
}
