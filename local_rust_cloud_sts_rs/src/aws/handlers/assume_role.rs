use std::fmt::Error;

use aws_sdk_sts::operation::assume_role::{AssumeRoleInput, AssumeRoleOutput};
use aws_sdk_sts::types::AssumedRoleUser;
use aws_smithy_types::date_time::Format;
use aws_smithy_xml::encode::XmlWriter;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::aws::handlers::action::Sts;
use crate::aws::handlers::constants::XMLNS;
use crate::aws::handlers::query::QueryReader;
use crate::aws::handlers::OutputWrapper;
use crate::secure;
use local_rust_cloud_sqlite::Database;

const PROPERTY_EXTERNAL_ID: &str = "ExternalId";
const PROPERTY_POLICY: &str = "Policy";
const PROPERTY_ROLE_ARN: &str = "RoleArn";
const PROPERTY_ROLE_SESSION_NAME: &str = "RoleSessionName";
const PROPERTY_DURATION_SECONDS: &str = "DurationSeconds";

const DEFAULT_DURATION_SECONDS: i32 = 3600;

pub type StsAssumeRoleOutput = OutputWrapper<AssumeRoleOutput>;

impl Sts {
    pub async fn assume_role<'a, I: Into<AssumeRoleInput>>(
        db: &Database, request_id: String, input: I,
    ) -> Result<StsAssumeRoleOutput, Error> {
        let input: AssumeRoleInput = input.into();
        let mut tx = db.new_tx().await.expect("failed to BEGIN a new transaction");
        let credentials_repo = crate::repository::CredentialsRepo::new();

        // todo: get role by ARN from IAM

        let assumed_role_user = AssumedRoleUser::builder()
            .set_arn(input.role_arn().map(|s| s.to_string()))
            .set_assumed_role_id(Some(format!("{}:{}", "AROA", input.role_session_name().unwrap_or("unknown"))))
            .build();

        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards");
        let expiration_seconds = i64::try_from(start_time.as_secs()).unwrap() + i64::from(input.duration_seconds().unwrap());
        let mut credentials = crate::models::Credentials::builder()
            .access_key_id(secure::generate_access_key())
            .secret_access_key(secure::generate_secret_access_key())
            .session_token(secure::generate_session_token())
            .expiration(expiration_seconds)
            // TODO: identify region and account from request
            .account_id(1)
            .region_id(1)
            .build();

        credentials_repo.create(&mut tx, &mut credentials).await;
        log::info!("credentials: {:?}", &credentials);

        let result = AssumeRoleOutput::builder()
            .set_assumed_role_user(Option::Some(assumed_role_user))
            .set_credentials(Option::Some(credentials.as_aws()))
            .set_packed_policy_size(Option::None)
            .set_source_identity(input.source_identity().map(|s| s.to_string()))
            .build();
        tx.commit().await.expect("failed to COMMIT transaction");

        Result::Ok(OutputWrapper::new(result, request_id))
    }
}

impl Into<AssumeRoleInput> for QueryReader {
    fn into(self) -> AssumeRoleInput {
        AssumeRoleInput::builder()
            .set_external_id(self.get_string(PROPERTY_EXTERNAL_ID))
            .set_role_arn(self.get_string(PROPERTY_ROLE_ARN))
            .set_role_session_name(self.get_string(PROPERTY_ROLE_SESSION_NAME))
            .set_duration_seconds(self.get_i32_or_default(PROPERTY_DURATION_SECONDS, DEFAULT_DURATION_SECONDS))
            .set_policy(self.get_string(PROPERTY_POLICY))
            .set_policy_arns(Option::None)
            .set_serial_number(Option::None)
            .set_token_code(Option::None)
            .set_source_identity(Option::None)
            .build()
            .unwrap()
    }
}

impl From<StsAssumeRoleOutput> for String {
    fn from(val: StsAssumeRoleOutput) -> Self {
        let mut out = String::new();
        let mut doc = XmlWriter::new(&mut out);
        let mut assume_role_response_tag = doc.start_el("AssumeRoleResponse").write_ns(XMLNS, None).finish();
        let mut assume_role_result_tag = assume_role_response_tag.start_el("AssumeRoleResult").finish();
        if val.inner.assumed_role_user().is_some() {
            let assumed_role_user = val.inner.assumed_role_user().unwrap();
            let mut assume_role_user_tag = assume_role_result_tag.start_el("AssumedRoleUser").finish();
            local_rust_cloud_xml::write_tag_with_value(&mut assume_role_user_tag, "Arn", assumed_role_user.arn());
            local_rust_cloud_xml::write_tag_with_value(&mut assume_role_user_tag, "AssumedRoleId", assumed_role_user.assumed_role_id());
            assume_role_user_tag.finish();
        }

        if val.inner.credentials().is_some() {
            let credentials = val.inner.credentials().unwrap();
            let mut credentials_tag = assume_role_result_tag.start_el("Credentials").finish();
            local_rust_cloud_xml::write_tag_with_value(&mut credentials_tag, "AccessKeyId", credentials.access_key_id());
            local_rust_cloud_xml::write_tag_with_value(&mut credentials_tag, "SecretAccessKey", credentials.secret_access_key());
            local_rust_cloud_xml::write_tag_with_value(&mut credentials_tag, "SessionToken", credentials.session_token());
            local_rust_cloud_xml::write_tag_with_value(
                &mut credentials_tag,
                "Expiration",
                credentials
                    .expiration()
                    .map(|date_time| date_time.fmt(Format::DateTime).expect("Failed to format date")),
            );
            credentials_tag.finish();
        }
        local_rust_cloud_xml::write_tag_with_value(
            &mut assume_role_result_tag,
            "PackedPolicySize",
            val.inner.packed_policy_size().map(|num| num.to_string()),
        );
        assume_role_result_tag.finish();

        let mut response_metadata_tag = assume_role_response_tag.start_el("ResponseMetadata").finish();
        local_rust_cloud_xml::write_tag_with_value(&mut response_metadata_tag, "RequestId", Option::Some(val.request_id));
        response_metadata_tag.finish();

        assume_role_response_tag.finish();
        return out;
    }
}
