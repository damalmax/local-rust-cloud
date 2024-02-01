use aws_sdk_iam::operation::create_instance_profile::CreateInstanceProfileOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalCreateInstanceProfileOutput = OutputWrapper<CreateInstanceProfileOutput>;

impl From<LocalCreateInstanceProfileOutput> for XmlResponse {
    fn from(val: LocalCreateInstanceProfileOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut create_instance_profile_response_tag = doc
            .start_el("CreateInstanceProfileResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut create_instance_profile_result_tag = create_instance_profile_response_tag
            .start_el("CreateInstanceProfileResult")
            .finish();
        if let Some(instance_profile) = val.inner.instance_profile() {
            let mut instance_profile_tag = create_instance_profile_result_tag.start_el("InstanceProfile").finish();

            local_cloud_xml::write_tag_with_value(
                &mut instance_profile_tag,
                "InstanceProfileName",
                Some(instance_profile.instance_profile_name()),
            );
            local_cloud_xml::write_tag_with_value(
                &mut instance_profile_tag,
                "InstanceProfileId",
                Some(instance_profile.instance_profile_id()),
            );
            local_cloud_xml::write_tag_with_value(&mut instance_profile_tag, "Arn", Some(instance_profile.arn()));
            local_cloud_xml::write_tag_with_value(&mut instance_profile_tag, "Path", Some(instance_profile.path()));
            local_cloud_xml::write_iso8061_datetime_value_tag(
                &mut instance_profile_tag,
                "CreateDate",
                Some(instance_profile.create_date()),
            );
            super::tags::write_slice(&mut instance_profile_tag, instance_profile.tags());

            instance_profile_tag.finish();
        }
        create_instance_profile_result_tag.finish();

        local_cloud_xml::write_request_metadata_tag(
            &mut create_instance_profile_response_tag,
            "ResponseMetadata",
            "RequestId",
            val.request_id,
        );

        create_instance_profile_response_tag.finish();
        return XmlResponse(out);
    }
}
