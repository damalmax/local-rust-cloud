use aws_sdk_iam::operation::create_user::CreateUserOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

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

        if let Some(user) = val.inner.user() {
            super::user::write(&mut create_user_result_tag, "User", user);
        }

        create_user_result_tag.finish();

        local_cloud_xml::write_request_metadata_tag(
            &mut create_user_response_tag,
            "ResponseMetadata",
            "RequestId",
            val.request_id,
        );

        create_user_response_tag.finish();
        XmlResponse(out)
    }
}
