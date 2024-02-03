use aws_sdk_iam::operation::list_instance_profiles_for_role::ListInstanceProfilesForRoleOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;
use local_cloud_xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalListInstanceProfilesForRoleOutput = OutputWrapper<ListInstanceProfilesForRoleOutput>;

impl From<LocalListInstanceProfilesForRoleOutput> for XmlResponse {
    fn from(val: LocalListInstanceProfilesForRoleOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("ListInstanceProfilesForRoleResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("ListInstanceProfilesForRoleResult").finish();

        super::instance_profiles::write_slice(&mut result_tag, "InstanceProfiles", val.inner.instance_profiles());
        if let Some(token) = val.inner.marker() {
            write_tag_with_value(&mut result_tag, "Marker", Some(token));
        }
        write_tag_with_value(&mut result_tag, "IsTruncated", Some(val.inner.is_truncated.to_string()));

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
