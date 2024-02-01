use aws_sdk_iam::operation::change_password::ChangePasswordOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalChangePasswordOutput = OutputWrapper<ChangePasswordOutput>;

impl From<LocalChangePasswordOutput> for XmlResponse {
    fn from(val: LocalChangePasswordOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut attach_group_policy_response_tag = doc
            .start_el("ChangePasswordResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let attach_group_policy_result_tag = attach_group_policy_response_tag
            .start_el("ChangePasswordResult")
            .finish();
        attach_group_policy_result_tag.finish();

        local_cloud_xml::write_request_metadata_tag(
            &mut attach_group_policy_response_tag,
            "ResponseMetadata",
            "RequestId",
            val.request_id,
        );

        attach_group_policy_response_tag.finish();
        XmlResponse(out)
    }
}
