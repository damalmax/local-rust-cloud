use std::time::{SystemTime, UNIX_EPOCH};

use aws_sdk_sts::operation::assume_role::AssumeRoleOutput;
use aws_sdk_sts::types::AssumedRoleUser;

use db::LocalDb;

use crate::http::aws::sts::actions::assume_role::LocalAssumeRole;
use crate::http::aws::sts::actions::error::{StsApiError, StsErrorKind};
use crate::http::aws::sts::actions::types::wrapper::OutputWrapper;
use crate::http::aws::sts::repository;
use crate::http::aws::sts::types::credentials::DbCredentials;
use crate::secure;

impl LocalAssumeRole {
    pub async fn execute(
        &self, account_id: i64, aws_request_id: &str, db: &LocalDb,
    ) -> Result<OutputWrapper<AssumeRoleOutput>, StsApiError> {
        let mut tx = db.new_tx().await.expect("failed to BEGIN a new transaction");
        // todo: get role by ARN from IAM

        let assumed_role_user = AssumedRoleUser::builder()
            .arn(self.role_arn().unwrap())
            .assumed_role_id(format!("{}:{}", "AROA", self.role_session_name().unwrap_or("unknown")))
            .build()
            .unwrap();

        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards");
        let expiration_seconds =
            i64::try_from(start_time.as_secs()).unwrap() + i64::from(self.duration_seconds().unwrap_or(90));
        let mut credentials = DbCredentials::builder()
            .access_key_id(secure::generate_access_key())
            .secret_access_key(secure::generate_secret_access_key())
            .session_token(secure::generate_session_token())
            .expiration(expiration_seconds)
            // TODO: identify region and account from request
            .account_id(account_id)
            .region_id(1)
            .build();

        repository::credentials::create(&mut tx, &mut credentials)
            .await
            .map_err(|_err| StsApiError {
                error_code: Default::default(),
                kind: StsErrorKind::InvalidInput,
                request_id: aws_request_id.to_owned(),
                message: "Failed to save credentials".to_owned(),
            })?;
        log::info!("credentials: {:?}", &credentials);

        let result = AssumeRoleOutput::builder()
            .assumed_role_user(assumed_role_user)
            .credentials(credentials.as_aws())
            .set_packed_policy_size(None)
            .source_identity(self.source_identity().unwrap_or(""))
            .build();
        tx.commit().await.expect("failed to COMMIT transaction");

        Ok(OutputWrapper::new(result, aws_request_id))
    }
}
