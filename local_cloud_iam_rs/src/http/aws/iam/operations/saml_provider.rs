use aws_sdk_iam::operation::create_saml_provider::CreateSamlProviderOutput;
use chrono::Utc;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::db;
use crate::http::aws::iam::db::types::saml_provider::InsertSamlProvider;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::create_saml_provider_request::CreateSamlProviderRequest;

pub(crate) async fn create_saml_provider(
    ctx: &OperationCtx, input: &CreateSamlProviderRequest, db: &LocalDb,
) -> Result<CreateSamlProviderOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let current_time = Utc::now().timestamp();
    let saml_provider_name = input.name().unwrap().trim();

    let mut insert_saml_provider = InsertSamlProvider {
        id: None,
        account_id: ctx.account_id,
        name: saml_provider_name.to_owned(),
        arn: format!("arn:aws:iam::{:0>12}:saml-provider/{saml_provider_name}", ctx.account_id),
        create_date: current_time,
        valid_until: None,
        metadata_document: input.saml_metadata_document().unwrap().to_owned(),
    };

    db::saml_provider::create(&mut tx, &mut insert_saml_provider).await?;

    let mut saml_provider_tags = super::common::prepare_tags_for_insert(input.tags(), insert_saml_provider.id.unwrap());

    db::saml_provider_tag::save_all(&mut tx, &mut saml_provider_tags).await?;

    let output = CreateSamlProviderOutput::builder()
        .saml_provider_arn(insert_saml_provider.arn)
        .set_tags(super::common::prepare_tags_for_output(&saml_provider_tags))
        .build();

    tx.commit().await?;
    Ok(output)
}
