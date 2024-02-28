use aws_sdk_iam::operation::simulate_custom_policy::SimulateCustomPolicyOutput;
use aws_sdk_iam::operation::simulate_principal_policy::SimulatePrincipalPolicyOutput;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::simulate_custom_policy::SimulateCustomPolicyRequest;
use crate::http::aws::iam::types::simulate_principal_policy::SimulatePrincipalPolicyRequest;

pub(crate) async fn simulate_custom_policy(
    ctx: &OperationCtx, input: &SimulateCustomPolicyRequest, db: &LocalDb,
) -> Result<SimulateCustomPolicyOutput, OperationError> {
    input.validate("$")?;

    let output = SimulateCustomPolicyOutput::builder().build();

    Ok(output)
}

pub(crate) async fn simulate_principal_policy(
    ctx: &OperationCtx, input: &SimulatePrincipalPolicyRequest, db: &LocalDb,
) -> Result<SimulatePrincipalPolicyOutput, OperationError> {
    input.validate("$")?;

    let output = SimulatePrincipalPolicyOutput::builder().build();

    Ok(output)
}
