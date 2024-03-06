use crate::tests::fixture::CREATE_USER_PERMISSIONS_BOUNDARY;

const GROUP_NAME: &str = "test_group";
const POLICY_NAME: &str = "test_policy";

#[tokio::test]
async fn delete_group_policy() {
    let ctx = local_cloud_testing::axum_suite::create_test_ctx(super::test_suite::start_server).await;
    let config = super::aws_config(ctx.port);
    let client = aws_sdk_iam::Client::new(&config);

    let _create_group_output = client
        .create_group()
        .group_name(GROUP_NAME)
        .path("/")
        .send()
        .await
        .expect("Failed to create IAM group");

    client
        .put_group_policy()
        .group_name(GROUP_NAME)
        .policy_name(POLICY_NAME)
        .policy_document(CREATE_USER_PERMISSIONS_BOUNDARY)
        .send()
        .await
        .expect("Failed to put group policy");

    client
        .delete_group_policy()
        .group_name(GROUP_NAME)
        .policy_name(POLICY_NAME)
        .send()
        .await
        .expect("Failed to delete group policy");

    ctx.stop_server().await;
}
