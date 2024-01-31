use aws_sdk_iam::operation::add_user_to_group::AddUserToGroupOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalAddUserToGroupOutput = OutputWrapper<AddUserToGroupOutput>;

impl From<LocalAddUserToGroupOutput> for XmlResponse {
    fn from(val: LocalAddUserToGroupOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut add_user_to_group_response_tag = doc
            .start_el("AddUserToGroupResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let add_user_to_group_result_tag = add_user_to_group_response_tag.start_el("AddUserToGroupResult").finish();
        add_user_to_group_result_tag.finish();

        local_cloud_xml::write_request_metadata_tag(
            &mut add_user_to_group_response_tag,
            "ResponseMetadata",
            "RequestId",
            val.request_id,
        );

        add_user_to_group_response_tag.finish();
        XmlResponse(out)
    }
}
