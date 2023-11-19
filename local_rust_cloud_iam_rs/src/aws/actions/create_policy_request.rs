use std::{iter, ops::Deref};

use aws_sdk_iam::{operation::create_policy::CreatePolicyInput, types::Tag};
use local_rust_cloud_common::request::LocalTag;

use super::query::QueryReader;

const PROPERTY_DESCRIPTION: &str = "Description";
const PROPERTY_PATH: &str = "Path";
const PROPERTY_POLICY_DOCUMENT: &str = "PolicyDocument";
const PROPERTY_POLICY_NAME: &str = "PolicyName";

#[derive(Debug)]
pub struct LocalCreatePolicyInput {
    inner: CreatePolicyInput,
    local_tags: Vec<LocalTag>,
}

impl Deref for LocalCreatePolicyInput {
    type Target = CreatePolicyInput;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl LocalCreatePolicyInput {
    pub fn local_tags(&self) -> &[LocalTag] {
        &self.local_tags
    }
}

impl Into<LocalCreatePolicyInput> for QueryReader {
    fn into(self) -> LocalCreatePolicyInput {
        let builder = CreatePolicyInput::builder()
            .set_description(self.get_string(PROPERTY_DESCRIPTION))
            .set_path(self.get_string(PROPERTY_PATH))
            .set_policy_document(self.get_string(PROPERTY_POLICY_DOCUMENT))
            .set_policy_name(self.get_string(PROPERTY_POLICY_NAME));

        let local_tags = self.get_tags();
        if local_tags.is_none() {
            return LocalCreatePolicyInput {
                inner: builder.build().unwrap(),
                local_tags: vec![],
            };
        }
        let local_tags = local_tags.unwrap();
        let mut tags: Vec<Tag> = vec![];
        for tag in &local_tags {
            tags.push(Tag::builder().key(tag.key()).value(tag.value()).build());
        }
        return LocalCreatePolicyInput {
            inner: builder.set_tags(Option::Some(tags)).build().unwrap(),
            local_tags: local_tags,
        };
    }
}
