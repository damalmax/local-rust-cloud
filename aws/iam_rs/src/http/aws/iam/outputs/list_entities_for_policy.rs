use aws_sdk_iam::operation::list_entities_for_policy::ListEntitiesForPolicyOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalListEntitiesForPolicyOutput = OutputWrapper<ListEntitiesForPolicyOutput>;

impl From<LocalListEntitiesForPolicyOutput> for XmlResponse {
    fn from(val: LocalListEntitiesForPolicyOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("ListEntitiesForPolicyResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("ListEntitiesForPolicyResult").finish();

        super::policy_groups::write_slice(&mut result_tag, "PolicyGroups", val.inner.policy_groups());
        super::policy_users::write_slice(&mut result_tag, "PolicyUsers", val.inner.policy_users());
        super::policy_roles::write_slice(&mut result_tag, "PolicyRoles", val.inner.policy_roles());
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
