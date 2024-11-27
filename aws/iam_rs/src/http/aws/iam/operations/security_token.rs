use aws_sdk_iam::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferencesOutput;
use sqlx::{Sqlite, Transaction};

use validators::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::set_security_token_service_preferences::SetSecurityTokenServicePreferencesRequest;

pub(crate) async fn set_security_token_service_preferences<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &SetSecurityTokenServicePreferencesRequest,
) -> Result<SetSecurityTokenServicePreferencesOutput, ActionError> {
    input.validate("$")?;

    let output = SetSecurityTokenServicePreferencesOutput::builder().build();
    Ok(output)
}
