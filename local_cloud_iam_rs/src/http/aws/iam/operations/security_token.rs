use aws_sdk_iam::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferencesOutput;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::set_security_token_service_preferences::SetSecurityTokenServicePreferencesRequest;

pub(crate) async fn set_security_token_service_preferences(
    ctx: &OperationCtx, input: &SetSecurityTokenServicePreferencesRequest, db: &LocalDb,
) -> Result<SetSecurityTokenServicePreferencesOutput, OperationError> {
    input.validate("$")?;

    let output = SetSecurityTokenServicePreferencesOutput::builder().build();
    Ok(output)
}
