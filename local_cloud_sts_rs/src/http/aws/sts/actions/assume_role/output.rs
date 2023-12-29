use aws_sdk_sts::operation::assume_role::AssumeRoleOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::sts::actions::types::wrapper::OutputWrapper;
use crate::http::aws::sts::constants;

pub type LocalAssumeRoleOutput = OutputWrapper<AssumeRoleOutput>;

impl From<LocalAssumeRoleOutput> for XmlResponse {
    fn from(val: LocalAssumeRoleOutput) -> Self {
        let mut out = String::new();
        let mut doc = XmlWriter::new(&mut out);
        let mut assume_role_response_tag = doc
            .start_el("AssumeRoleResponse")
            .write_ns(constants::xml::STS_XMLNS, None)
            .finish();
        let mut assume_role_result_tag = assume_role_response_tag.start_el("AssumeRoleResult").finish();
        if val.inner.assumed_role_user().is_some() {
            let assumed_role_user = val.inner.assumed_role_user().unwrap();
            let mut assume_role_user_tag = assume_role_result_tag.start_el("AssumedRoleUser").finish();
            local_cloud_xml::write_tag_with_value(&mut assume_role_user_tag, "Arn", Some(assumed_role_user.arn()));
            local_cloud_xml::write_tag_with_value(
                &mut assume_role_user_tag,
                "AssumedRoleId",
                Some(assumed_role_user.assumed_role_id()),
            );
            assume_role_user_tag.finish();
        }

        if val.inner.credentials().is_some() {
            let credentials = val.inner.credentials().unwrap();
            let mut credentials_tag = assume_role_result_tag.start_el("Credentials").finish();
            local_cloud_xml::write_tag_with_value(
                &mut credentials_tag,
                "AccessKeyId",
                Some(credentials.access_key_id()),
            );
            local_cloud_xml::write_tag_with_value(
                &mut credentials_tag,
                "SecretAccessKey",
                Some(credentials.secret_access_key()),
            );
            local_cloud_xml::write_tag_with_value(
                &mut credentials_tag,
                "SessionToken",
                Some(credentials.session_token()),
            );
            local_cloud_xml::write_iso8061_datetime_value_tag(
                &mut credentials_tag,
                "Expiration",
                Some(credentials.expiration()),
            );
            credentials_tag.finish();
        }
        local_cloud_xml::write_tag_with_value(
            &mut assume_role_result_tag,
            "PackedPolicySize",
            val.inner.packed_policy_size().map(|num| num.to_string()),
        );
        assume_role_result_tag.finish();

        let mut response_metadata_tag = assume_role_response_tag.start_el("ResponseMetadata").finish();
        local_cloud_xml::write_tag_with_value(&mut response_metadata_tag, "RequestId", Option::Some(val.request_id));
        response_metadata_tag.finish();

        assume_role_response_tag.finish();
        return XmlResponse(out);
    }
}
