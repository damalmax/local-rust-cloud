use aws_sdk_iam::operation::create_login_profile::CreateLoginProfileOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::write_request_metadata_tag;

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalCreateLoginProfileOutput = OutputWrapper<CreateLoginProfileOutput>;

impl From<LocalCreateLoginProfileOutput> for XmlResponse {
    fn from(val: LocalCreateLoginProfileOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("CreateLoginProfileResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("CreateLoginProfileResult").finish();

        if let Some(login_profile) = val.inner.login_profile() {
            let mut login_profile_tag = result_tag.start_el("LoginProfile").finish();
            xml::write_tag_with_value(&mut login_profile_tag, "UserName", Some(login_profile.user_name()));
            xml::write_tag_with_value(
                &mut login_profile_tag,
                "PasswordResetRequired",
                Some(login_profile.password_reset_required().to_string()),
            );
            xml::write_iso8061_datetime_value_tag(
                &mut login_profile_tag,
                "CreateDate",
                Some(login_profile.create_date()),
            );
            login_profile_tag.finish();
        }

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
