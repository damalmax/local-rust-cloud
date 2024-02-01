use aws_sdk_iam::operation::create_login_profile::CreateLoginProfileOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalCreateLoginProfileOutput = OutputWrapper<CreateLoginProfileOutput>;

impl From<LocalCreateLoginProfileOutput> for XmlResponse {
    fn from(val: LocalCreateLoginProfileOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut create_login_profile_response_tag = doc
            .start_el("CreateLoginProfileResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut create_login_profile_result_tag = create_login_profile_response_tag
            .start_el("CreateLoginProfileResult")
            .finish();

        if let Some(login_profile) = val.inner.login_profile() {
            let mut login_profile_tag = create_login_profile_result_tag.start_el("LoginProfile").finish();
            local_cloud_xml::write_tag_with_value(&mut login_profile_tag, "UserName", Some(login_profile.user_name()));
            local_cloud_xml::write_tag_with_value(
                &mut login_profile_tag,
                "PasswordResetRequired",
                Some(login_profile.password_reset_required().to_string()),
            );
            local_cloud_xml::write_iso8061_datetime_value_tag(
                &mut login_profile_tag,
                "CreateDate",
                Some(login_profile.create_date()),
            );
            login_profile_tag.finish();
        }

        create_login_profile_result_tag.finish();

        local_cloud_xml::write_request_metadata_tag(
            &mut create_login_profile_response_tag,
            "ResponseMetadata",
            "RequestId",
            val.request_id,
        );

        create_login_profile_response_tag.finish();
        XmlResponse(out)
    }
}
