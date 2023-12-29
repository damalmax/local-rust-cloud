use aws_sdk_iam::operation::create_user::CreateUserOutput;
use aws_sdk_iam::types::Tag;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::actions::wrapper::OutputWrapper;
use crate::http::aws::iam::constants;

pub type LocalCreateUserOutput = OutputWrapper<CreateUserOutput>;

impl From<LocalCreateUserOutput> for XmlResponse {
    fn from(val: LocalCreateUserOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut create_user_response_tag = doc
            .start_el("CreateUserResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut create_user_result_tag = create_user_response_tag.start_el("CreateUserResult").finish();

        if val.inner.user().is_some() {
            let user = val.inner.user().unwrap();
            let mut user_tag = create_user_result_tag.start_el("User").finish();
            local_cloud_xml::write_key_value_tags(
                &mut user_tag,
                user.tags(),
                |t: &Tag| Some(t.key().to_owned()),
                |t: &Tag| Some(t.value().to_owned()),
            );
            user_tag.finish();
        }

        create_user_result_tag.finish();

        let mut response_metadata_tag = create_user_response_tag.start_el("ResponseMetadata").finish();
        local_cloud_xml::write_tag_with_value(&mut response_metadata_tag, "RequestId", Some(val.request_id));
        response_metadata_tag.finish();

        create_user_response_tag.finish();
        return XmlResponse(out);
    }
}
