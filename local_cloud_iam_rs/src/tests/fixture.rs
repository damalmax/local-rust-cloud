use aws_sdk_iam::operation::create_policy::{CreatePolicyError, CreatePolicyOutput};
use aws_sdk_iam::operation::create_user::{CreateUserError, CreateUserOutput};
use aws_sdk_iam::types::Tag;
use aws_smithy_runtime_api::client::orchestrator::HttpResponse;
use aws_smithy_runtime_api::client::result::SdkError;

pub(crate) const CREATE_USER_PERMISSIONS_BOUNDARY: &str =
    include_str!("resources/create_user__permissions_boundary.json");

pub(crate) const CREATE_ROLE_ASSUME_ROLE_PERMISSIONS_BOUNDARY: &str =
    include_str!("resources/create_role__assume_role_policy_document.json");

pub(crate) fn tag(key: &str, value: &str) -> Tag {
    Tag::builder().key(key).value(value).build().unwrap()
}

pub(crate) async fn create_policy(
    client: &aws_sdk_iam::Client, policy_name: &str, description: &str, path: &str, policy_document: &str,
    tags: Option<Vec<Tag>>,
) -> Result<CreatePolicyOutput, SdkError<CreatePolicyError, HttpResponse>> {
    client
        .create_policy()
        .description(description)
        .path(path)
        .policy_document(policy_document)
        .policy_name(policy_name)
        .set_tags(tags)
        .send()
        .await
}

pub(crate) async fn create_user(
    client: &aws_sdk_iam::Client, user_name: &str, path: &str, policy_arn: Option<&str>, tags: Option<Vec<Tag>>,
) -> Result<CreateUserOutput, SdkError<CreateUserError, HttpResponse>> {
    client
        .create_user()
        .user_name(user_name)
        .path(path)
        .set_permissions_boundary(policy_arn.map(|s| s.to_owned()))
        .set_tags(tags)
        .send()
        .await
}
