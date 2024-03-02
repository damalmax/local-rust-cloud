use aws_sdk_iam::operation::generate_credential_report::GenerateCredentialReportOutput;
use aws_sdk_iam::operation::generate_organizations_access_report::GenerateOrganizationsAccessReportOutput;
use aws_sdk_iam::operation::get_credential_report::GetCredentialReportOutput;
use aws_sdk_iam::operation::get_organizations_access_report::GetOrganizationsAccessReportOutput;
use sqlx::{Sqlite, Transaction};

use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::generate_credential_report::GenerateCredentialReportRequest;
use crate::http::aws::iam::types::generate_organizations_access_report::GenerateOrganizationsAccessReportRequest;
use crate::http::aws::iam::types::get_credential_report::GetCredentialReportRequest;
use crate::http::aws::iam::types::get_organizations_access_report::GetOrganizationsAccessReportRequest;

pub(crate) async fn get_credential_report<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetCredentialReportRequest,
) -> Result<GetCredentialReportOutput, ActionError> {
    input.validate("$")?;

    let output = GetCredentialReportOutput::builder().build();
    Ok(output)
}

pub(crate) async fn generate_credential_report<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GenerateCredentialReportRequest,
) -> Result<GenerateCredentialReportOutput, ActionError> {
    input.validate("$")?;

    let output = GenerateCredentialReportOutput::builder().build();
    Ok(output)
}

pub(crate) async fn generate_organizations_access_report<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GenerateOrganizationsAccessReportRequest,
) -> Result<GenerateOrganizationsAccessReportOutput, ActionError> {
    input.validate("$")?;

    let output = GenerateOrganizationsAccessReportOutput::builder().build();
    Ok(output)
}

pub(crate) async fn get_organizations_access_report<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetOrganizationsAccessReportRequest,
) -> Result<GetOrganizationsAccessReportOutput, ActionError> {
    input.validate("$")?;

    let output = GetOrganizationsAccessReportOutput::builder().build().unwrap();
    Ok(output)
}
