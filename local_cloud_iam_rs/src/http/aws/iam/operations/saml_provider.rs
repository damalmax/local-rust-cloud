use aws_sdk_iam::operation::create_saml_provider::CreateSamlProviderOutput;
use aws_sdk_iam::operation::delete_saml_provider::DeleteSamlProviderOutput;
use aws_sdk_iam::operation::get_saml_provider::GetSamlProviderOutput;
use aws_sdk_iam::operation::list_saml_provider_tags::ListSamlProviderTagsOutput;
use aws_sdk_iam::operation::list_saml_providers::ListSamlProvidersOutput;
use aws_sdk_iam::operation::tag_saml_provider::TagSamlProviderOutput;
use aws_sdk_iam::operation::untag_saml_provider::UntagSamlProviderOutput;
use aws_sdk_iam::operation::update_saml_provider::UpdateSamlProviderOutput;
use aws_sdk_iam::types::SamlProviderListEntry;
use aws_smithy_types::DateTime;
use chrono::Utc;
use sqlx::{Executor, Sqlite, Transaction};

use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::saml_provider::{InsertSamlProvider, SelectSamlProvider};
use crate::http::aws::iam::db::types::tags::ListTagsQuery;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::create_saml_provider::CreateSamlProviderRequest;
use crate::http::aws::iam::types::delete_saml_provider::DeleteSamlProviderRequest;
use crate::http::aws::iam::types::get_saml_provider::GetSamlProviderRequest;
use crate::http::aws::iam::types::list_saml_provider_tags::ListSamlProviderTagsRequest;
use crate::http::aws::iam::types::list_saml_providers::ListSamlProvidersRequest;
use crate::http::aws::iam::types::tag_saml_provider::TagSamlProviderRequest;
use crate::http::aws::iam::types::untag_saml_provider::UntagSamlProviderRequest;
use crate::http::aws::iam::types::update_saml_provider::UpdateSamlProviderRequest;
use crate::http::aws::iam::{constants, db};

pub(crate) async fn find_id_by_arn<'a, E>(executor: E, account_id: i64, arn: &str) -> Result<i64, ActionError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::saml_provider::find_id_by_arn(executor, account_id, arn).await? {
        Some(provider_id) => Ok(provider_id),
        None => {
            return Err(ActionError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM SAML provider with ARN '{}' doesn't exist.", arn).as_str(),
            ));
        }
    }
}

pub(crate) async fn find_by_arn<'a, E>(
    executor: E, account_id: i64, arn: &str,
) -> Result<SelectSamlProvider, ActionError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::saml_provider::find_by_arn(executor, account_id, arn).await? {
        Some(provider) => Ok(provider),
        None => {
            return Err(ActionError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM SAML provider with ARN '{}' doesn't exist.", arn).as_str(),
            ));
        }
    }
}

pub(crate) async fn create_saml_provider<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &CreateSamlProviderRequest,
) -> Result<CreateSamlProviderOutput, ActionError> {
    input.validate("$")?;

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

    db::saml_provider::create(tx, &mut insert_saml_provider).await?;

    let mut saml_provider_tags = super::tag::prepare_for_db(input.tags(), insert_saml_provider.id.unwrap());

    db::Tags::SamlProvider.save_all(tx, &mut saml_provider_tags).await?;

    let output = CreateSamlProviderOutput::builder()
        .saml_provider_arn(insert_saml_provider.arn)
        .set_tags(super::tag::prepare_for_output(&saml_provider_tags))
        .build();
    Ok(output)
}

pub(crate) async fn tag_saml_provider<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &TagSamlProviderRequest,
) -> Result<TagSamlProviderOutput, ActionError> {
    input.validate("$")?;

    let saml_provider_id = find_id_by_arn(tx.as_mut(), ctx.account_id, input.saml_provider_arn().unwrap()).await?;
    let mut saml_provider_tags = super::tag::prepare_for_db(input.tags(), saml_provider_id);

    db::Tags::SamlProvider.save_all(tx, &mut saml_provider_tags).await?;
    let count = db::Tags::SamlProvider.count(tx.as_mut(), saml_provider_id).await?;
    if count > constants::tag::MAX_COUNT {
        return Err(ActionError::new(
            ApiErrorKind::LimitExceeded,
            format!("Cannot assign more than {} tags to IAM SAML provider.", constants::tag::MAX_COUNT).as_str(),
        ));
    }

    let output = TagSamlProviderOutput::builder().build();
    Ok(output)
}

pub(crate) async fn list_saml_provider_tags<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListSamlProviderTagsRequest,
) -> Result<ListSamlProviderTagsOutput, ActionError> {
    input.validate("$")?;

    let provider_id = find_id_by_arn(tx.as_mut(), ctx.account_id, input.saml_provider_arn().unwrap()).await?;

    let query = ListTagsQuery::new(input.max_items(), input.marker_type());
    let found_tags = db::Tags::SamlProvider.list(tx.as_mut(), provider_id, &query).await?;

    let tags = super::common::convert_and_limit(&found_tags, query.limit);
    let marker = super::common::create_encoded_marker(&query, found_tags.len())?;

    let output = ListSamlProviderTagsOutput::builder()
        .set_tags(tags)
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build()
        .unwrap();
    Ok(output)
}

pub(crate) async fn untag_saml_provider<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &UntagSamlProviderRequest,
) -> Result<UntagSamlProviderOutput, ActionError> {
    input.validate("$")?;

    let saml_provider_id =
        find_id_by_arn(tx.as_mut(), ctx.account_id, input.saml_provider_arn().unwrap().trim()).await?;

    db::Tags::SamlProvider
        .delete_all(tx, saml_provider_id, &input.tag_keys())
        .await?;

    let output = UntagSamlProviderOutput::builder().build();
    Ok(output)
}

pub(crate) async fn list_saml_providers<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListSamlProvidersRequest,
) -> Result<ListSamlProvidersOutput, ActionError> {
    input.validate("$")?;

    let result_vec = db::saml_provider::list(tx.as_mut(), ctx.account_id).await?;

    let mut saml_provider_list = vec![];
    for provider in result_vec {
        saml_provider_list.push(
            SamlProviderListEntry::builder()
                .arn(&provider.arn)
                .create_date(DateTime::from_secs(provider.create_date))
                .set_valid_until(provider.valid_until.map(DateTime::from_secs))
                .build(),
        )
    }
    let output = ListSamlProvidersOutput::builder()
        .set_saml_provider_list(Some(saml_provider_list))
        .build();
    Ok(output)
}

pub(crate) async fn update_saml_provider<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &UpdateSamlProviderRequest,
) -> Result<UpdateSamlProviderOutput, ActionError> {
    input.validate("$")?;

    let arn = input.saml_provider_arn().unwrap();
    let metadata_document = input.saml_metadata_document().unwrap();
    let is_updated = db::saml_provider::update_metadata(tx.as_mut(), ctx.account_id, arn, metadata_document).await?;
    if !is_updated {
        return Err(ActionError::new(
            ApiErrorKind::NoSuchEntity,
            format!("IAM SAML provider with ARN '{}' doesn't exist.", arn).as_str(),
        ));
    }

    let output = UpdateSamlProviderOutput::builder().build();
    Ok(output)
}

pub(crate) async fn get_saml_provider<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetSamlProviderRequest,
) -> Result<GetSamlProviderOutput, ActionError> {
    input.validate("$")?;

    let arn = input.saml_provider_arn().unwrap();
    let provider = find_by_arn(tx.as_mut(), ctx.account_id, arn).await?;

    let output = GetSamlProviderOutput::builder()
        .create_date(DateTime::from_secs(provider.create_date))
        .set_valid_until(provider.valid_until.map(DateTime::from_secs))
        .saml_metadata_document(&provider.metadata_document)
        .build();
    Ok(output)
}

pub(crate) async fn delete_saml_provider<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &DeleteSamlProviderRequest,
) -> Result<DeleteSamlProviderOutput, ActionError> {
    input.validate("$")?;

    let output = DeleteSamlProviderOutput::builder().build();
    Ok(output)
}
