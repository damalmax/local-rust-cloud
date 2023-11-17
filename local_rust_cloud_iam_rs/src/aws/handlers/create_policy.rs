use std::fmt::Error;

use aws_sdk_iam::{
    operation::create_policy::{CreatePolicyInput, CreatePolicyOutput},
    types::{Policy, Tag},
};
use aws_smithy_xml::encode::XmlWriter;
use local_rust_cloud_sqlite::Database;

use super::{action::Iam, constants::XMLNS, query::QueryReader, OutputWrapper};

const PROPERTY_DESCRIPTION: &str = "Description";
const PROPERTY_PATH: &str = "Path";
const PROPERTY_POLICY_DOCUMENT: &str = "PolicyDocument";
const PROPERTY_POLICY_NAME: &str = "PolicyName";

pub type IamCreatePolicyOutput = OutputWrapper<CreatePolicyOutput>;

impl Iam {
    pub async fn create_policy<'a, I: Into<CreatePolicyInput>>(
        db: &Database, request_id: String, input: I,
    ) -> Result<IamCreatePolicyOutput, Error> {
        let input: CreatePolicyInput = input.into();

        let policy_builder = Policy::builder().policy_name(input.policy_name().unwrap());
        let mut tags = vec![];
        for tag in input.tags().unwrap() {
            let tag = Tag::builder()
                .key(tag.key().map(|v| v.to_string()).unwrap_or(String::new()))
                .value(tag.value().map(|v| v.to_string()).unwrap_or(String::new()))
                .build();
            tags.push(tag);
        }
        let policy = policy_builder.set_tags(Option::Some(tags)).build();
        let result = CreatePolicyOutput::builder().policy(policy).build();
        Result::Ok(OutputWrapper::new(result, request_id))
    }
}

impl Into<CreatePolicyInput> for QueryReader {
    fn into(self) -> CreatePolicyInput {
        let builder = CreatePolicyInput::builder()
            .set_description(self.get_string(PROPERTY_DESCRIPTION))
            .set_path(self.get_string(PROPERTY_PATH))
            .set_policy_document(self.get_string(PROPERTY_POLICY_DOCUMENT))
            .set_policy_name(self.get_string(PROPERTY_POLICY_NAME));

        let tags = self.get_tags();
        if tags.is_none() {
            builder.set_tags(Option::None).build().unwrap()
        } else {
            let mut input_tags: Vec<Tag> = vec![];
            for tag in tags.unwrap() {
                input_tags.push(Tag::builder().key(tag.key).value(tag.value.unwrap_or(String::new())).build());
            }
            builder.set_tags(Option::Some(input_tags)).build().unwrap()
        }
    }
}

impl From<IamCreatePolicyOutput> for String {
    fn from(val: IamCreatePolicyOutput) -> Self {
        let mut out = String::new();
        let mut doc = XmlWriter::new(&mut out);

        let mut create_policy_response_tag = doc.start_el("CreatePolicyResponse").write_ns(XMLNS, None).finish();

        let mut create_policy_result_tag = create_policy_response_tag.start_el("CreatePolicyResult").finish();
        if val.inner.policy().is_some() {
            let policy = val.inner.policy().unwrap();
            let mut policy_tag = create_policy_result_tag.start_el("Policy").finish();
            local_rust_cloud_xml::write_tag_with_value(&mut policy_tag, "PolicyName", policy.policy_name());
            local_rust_cloud_xml::write_tag_with_value(&mut policy_tag, "PolicyId", policy.policy_id());
            local_rust_cloud_xml::write_tag_with_value(&mut policy_tag, "Arn", policy.arn());
            local_rust_cloud_xml::write_tag_with_value(&mut policy_tag, "Path", policy.path());
            local_rust_cloud_xml::write_tag_with_value(&mut policy_tag, "DefaultVersionId", policy.default_version_id());
            local_rust_cloud_xml::write_tag_with_value(
                &mut policy_tag,
                "AttachmentCount",
                policy.attachment_count().map(|v| v.to_string()),
            );
            local_rust_cloud_xml::write_tag_with_value(
                &mut policy_tag,
                "PermissionsBoundaryUsageCount",
                policy.permissions_boundary_usage_count().map(|v| v.to_string()),
            );
            local_rust_cloud_xml::write_tag_with_value(&mut policy_tag, "IsAttachable", Option::Some(policy.is_attachable().to_string()));
            local_rust_cloud_xml::write_tag_with_value(&mut policy_tag, "Description", policy.description());
            local_rust_cloud_xml::write_tag_with_date_value(&mut policy_tag, "CreateDate", policy.create_date());
            local_rust_cloud_xml::write_tag_with_date_value(&mut policy_tag, "UpdateDate", policy.update_date());
            local_rust_cloud_xml::write_key_value_tags(
                &mut policy_tag,
                policy.tags(),
                |t: &Tag| t.key().map(|v| v.to_string()),
                |t: &Tag| t.value().map(|v| v.to_string()),
            );
        }
        create_policy_result_tag.finish();

        let mut response_metadata_tag = create_policy_response_tag.start_el("ResponseMetadata").finish();
        local_rust_cloud_xml::write_tag_with_value(&mut response_metadata_tag, "RequestId", Option::Some(val.request_id));
        response_metadata_tag.finish();

        create_policy_response_tag.finish();
        return out;
    }
}
