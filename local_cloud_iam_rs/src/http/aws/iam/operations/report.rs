use aws_sdk_iam::operation::generate_credential_report::GenerateCredentialReportOutput;
use aws_sdk_iam::operation::generate_organizations_access_report::GenerateOrganizationsAccessReportOutput;
use aws_sdk_iam::operation::get_credential_report::GetCredentialReportOutput;
use aws_sdk_iam::operation::get_organizations_access_report::GetOrganizationsAccessReportOutput;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::generate_credential_report::GenerateCredentialReportRequest;
use crate::http::aws::iam::types::generate_organizations_access_report::GenerateOrganizationsAccessReportRequest;
use crate::http::aws::iam::types::get_credential_report::GetCredentialReportRequest;
use crate::http::aws::iam::types::get_organizations_access_report::GetOrganizationsAccessReportRequest;

pub(crate) async fn get_credential_report(
    ctx: &OperationCtx, input: &GetCredentialReportRequest, db: &LocalDb,
) -> Result<GetCredentialReportOutput, OperationError> {
    input.validate("$")?;

    let output = GetCredentialReportOutput::builder().build();

    Ok(output)
}

pub(crate) async fn generate_credential_report(
    ctx: &OperationCtx, input: &GenerateCredentialReportRequest, db: &LocalDb,
) -> Result<GenerateCredentialReportOutput, OperationError> {
    input.validate("$")?;

    let output = GenerateCredentialReportOutput::builder().build();

    Ok(output)
}

pub(crate) async fn generate_organizations_access_report(
    ctx: &OperationCtx, input: &GenerateOrganizationsAccessReportRequest, db: &LocalDb,
) -> Result<GenerateOrganizationsAccessReportOutput, OperationError> {
    input.validate("$")?;

    let output = GenerateOrganizationsAccessReportOutput::builder().build();

    Ok(output)
}

pub(crate) async fn get_organizations_access_report(
    ctx: &OperationCtx, input: &GetOrganizationsAccessReportRequest, db: &LocalDb,
) -> Result<GetOrganizationsAccessReportOutput, OperationError> {
    input.validate("$")?;

    let output = GetOrganizationsAccessReportOutput::builder().build().unwrap();

    Ok(output)
}
