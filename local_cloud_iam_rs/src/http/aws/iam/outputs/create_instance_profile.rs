use aws_sdk_iam::operation::create_instance_profile::CreateInstanceProfileOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;
use local_cloud_xml::{write_iso8061_datetime_value_tag, write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalCreateInstanceProfileOutput = OutputWrapper<CreateInstanceProfileOutput>;

impl From<LocalCreateInstanceProfileOutput> for XmlResponse {
    fn from(val: LocalCreateInstanceProfileOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("CreateInstanceProfileResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("CreateInstanceProfileResult").finish();
        if let Some(instance_profile) = val.inner.instance_profile() {
            let mut instance_profile_tag = result_tag.start_el("InstanceProfile").finish();

            write_tag_with_value(
                &mut instance_profile_tag,
                "InstanceProfileName",
                Some(instance_profile.instance_profile_name()),
            );
            write_tag_with_value(
                &mut instance_profile_tag,
                "InstanceProfileId",
                Some(instance_profile.instance_profile_id()),
            );
            write_tag_with_value(&mut instance_profile_tag, "Arn", Some(instance_profile.arn()));
            write_tag_with_value(&mut instance_profile_tag, "Path", Some(instance_profile.path()));
            write_iso8061_datetime_value_tag(
                &mut instance_profile_tag,
                "CreateDate",
                Some(instance_profile.create_date()),
            );
            super::tags::write_slice(&mut instance_profile_tag, instance_profile.tags());

            instance_profile_tag.finish();
        }
        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        return XmlResponse(out);
    }
}
