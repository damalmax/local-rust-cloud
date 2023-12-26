use aws_sdk_iam::operation::create_user::CreateUserOutput;
use aws_smithy_xml::encode::XmlWriter;

use super::{constants::IAM_XMLNS, response::IamResponse, OutputWrapper};

pub type LocalCreateUserOutput = OutputWrapper<CreateUserOutput>;

impl From<LocalCreateUserOutput> for IamResponse {
    fn from(val: LocalCreateUserOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut create_user_response_tag = doc.start_el("CreateUserResponse").write_ns(IAM_XMLNS, None).finish();

        let mut create_user_result_tag = create_user_response_tag.start_el("CreateUserResult").finish();

        if val.inner.user().is_some() {
            let user = val.inner.user().unwrap();
            let mut user_tag = create_user_result_tag.start_el("User").finish();

            if user.tags().len() > 0 {
                let mut tags_tag = user_tag.start_el("Tags").finish();
                let tags = user.tags();
                for tag in tags {
                    let mut tag_tag = tags_tag.start_el("Tag.member.0").finish();
                    local_cloud_xml::write_tag_with_value(&mut tag_tag, "Key", Some(tag.key()));
                    local_cloud_xml::write_tag_with_value(&mut tag_tag, "Value", Some(tag.value()));
                    tag_tag.finish();
                }
                tags_tag.finish();
            }

            user_tag.finish();
        }

        create_user_result_tag.finish();

        let mut response_metadata_tag = create_user_response_tag.start_el("ResponseMetadata").finish();
        local_cloud_xml::write_tag_with_value(&mut response_metadata_tag, "RequestId", Some(val.request_id));
        response_metadata_tag.finish();

        create_user_response_tag.finish();
        return out;
    }
}
