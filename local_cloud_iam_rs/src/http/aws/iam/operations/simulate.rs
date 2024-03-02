use aws_sdk_iam::operation::simulate_custom_policy::SimulateCustomPolicyOutput;
use aws_sdk_iam::operation::simulate_principal_policy::SimulatePrincipalPolicyOutput;
use sqlx::{Sqlite, Transaction};

use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::simulate_custom_policy::SimulateCustomPolicyRequest;
use crate::http::aws::iam::types::simulate_principal_policy::SimulatePrincipalPolicyRequest;

pub(crate) async fn simulate_custom_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &SimulateCustomPolicyRequest,
) -> Result<SimulateCustomPolicyOutput, ActionError> {
    input.validate("$")?;

    let output = SimulateCustomPolicyOutput::builder().build();
    Ok(output)
}

pub(crate) async fn simulate_principal_policy<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &SimulatePrincipalPolicyRequest,
) -> Result<SimulatePrincipalPolicyOutput, ActionError> {
    input.validate("$")?;

    let output = SimulatePrincipalPolicyOutput::builder().build();
    Ok(output)
}
