use aws_sdk_iam::operation::get_instance_profile::GetInstanceProfileOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetInstanceProfileOutput = OutputWrapper<GetInstanceProfileOutput>;

impl From<LocalGetInstanceProfileOutput> for XmlResponse {
    fn from(val: LocalGetInstanceProfileOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetInstanceProfileResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetInstanceProfileResult").finish();
        if let Some(profile) = val.inner.instance_profile() {
            super::instance_profiles::write(&mut result_tag, "InstanceProfile", profile);
        }
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
