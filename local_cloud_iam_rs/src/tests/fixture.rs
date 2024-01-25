use aws_sdk_iam::operation::create_policy::{CreatePolicyError, CreatePolicyOutput};
use aws_sdk_iam::types::Tag;
use aws_smithy_runtime_api::client::orchestrator::HttpResponse;
use aws_smithy_runtime_api::client::result::SdkError;

pub(crate) const CREATE_USER_PERMISSIONS_BOUNDARY: &str =
    include_str!("resources/create_user__permissions_boundary.json");

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
