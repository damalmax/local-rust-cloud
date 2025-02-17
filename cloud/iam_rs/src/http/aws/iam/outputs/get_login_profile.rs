use aws_sdk_iam::operation::get_login_profile::GetLoginProfileOutput;
use aws_smithy_xml::encode::XmlWriter;

use web::local::XmlResponse;
use xml::{write_iso8061_datetime_value_tag, write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetLoginProfileOutput = OutputWrapper<GetLoginProfileOutput>;

impl From<LocalGetLoginProfileOutput> for XmlResponse {
    fn from(val: LocalGetLoginProfileOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetLoginProfileResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetLoginProfileResult").finish();

        if let Some(profile) = val.inner.login_profile() {
            let mut profile_tag = result_tag.start_el("LoginProfile").finish();
            write_tag_with_value(&mut profile_tag, "UserName", Some(profile.user_name()));
            write_tag_with_value(
                &mut profile_tag,
                "PasswordResetRequired",
                Some(profile.password_reset_required().to_string()),
            );
            write_iso8061_datetime_value_tag(&mut profile_tag, "CreateDate", Some(profile.create_date()));
            profile_tag.finish();
        }

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
