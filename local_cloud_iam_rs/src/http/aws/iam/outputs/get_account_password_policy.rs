use aws_sdk_iam::operation::get_account_password_policy::GetAccountPasswordPolicyOutput;
use aws_smithy_xml::encode::XmlWriter;

use local_cloud_axum::local::web::XmlResponse;
use local_cloud_xml::{write_request_metadata_tag, write_tag_with_value};

use crate::http::aws::iam::constants;
use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalGetAccountPasswordPolicyOutput = OutputWrapper<GetAccountPasswordPolicyOutput>;

impl From<LocalGetAccountPasswordPolicyOutput> for XmlResponse {
    fn from(val: LocalGetAccountPasswordPolicyOutput) -> Self {
        let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
        let mut doc = XmlWriter::new(&mut out);

        let mut response_tag = doc
            .start_el("GetAccountPasswordPolicyResponse")
            .write_ns(constants::xml::IAM_XMLNS, None)
            .finish();

        let mut result_tag = response_tag.start_el("GetAccountPasswordPolicyResult").finish();

        if let Some(password_policy) = val.inner.password_policy() {
            let mut policy_tag = result_tag.start_el("PasswordPolicy").finish();
            write_tag_with_value(
                &mut policy_tag,
                "AllowUsersToChangePassword",
                Some(password_policy.allow_users_to_change_password().to_string()),
            );
            write_tag_with_value(
                &mut policy_tag,
                "RequireUppercaseCharacters",
                Some(password_policy.require_uppercase_characters().to_string()),
            );
            write_tag_with_value(
                &mut policy_tag,
                "RequireSymbols",
                Some(password_policy.require_symbols().to_string()),
            );
            write_tag_with_value(
                &mut policy_tag,
                "ExpirePasswords",
                Some(password_policy.expire_passwords().to_string()),
            );
            write_tag_with_value(
                &mut policy_tag,
                "PasswordReusePrevention",
                password_policy.password_reuse_prevention().map(|v| v.to_string()),
            );
            write_tag_with_value(
                &mut policy_tag,
                "RequireLowercaseCharacters",
                Some(password_policy.require_lowercase_characters().to_string()),
            );
            write_tag_with_value(
                &mut policy_tag,
                "MaxPasswordAge",
                password_policy.max_password_age().map(|v| v.to_string()),
            );
            write_tag_with_value(&mut policy_tag, "HardExpiry", password_policy.hard_expiry().map(|v| v.to_string()));
            write_tag_with_value(
                &mut policy_tag,
                "RequireNumbers",
                Some(password_policy.require_numbers().to_string()),
            );
            write_tag_with_value(
                &mut policy_tag,
                "MinimumPasswordLength",
                password_policy.minimum_password_length().map(|v| v.to_string()),
            );
            policy_tag.finish();
        }

        result_tag.finish();

        write_request_metadata_tag(&mut response_tag, "ResponseMetadata", "RequestId", val.request_id);

        response_tag.finish();
        XmlResponse(out)
    }
}
