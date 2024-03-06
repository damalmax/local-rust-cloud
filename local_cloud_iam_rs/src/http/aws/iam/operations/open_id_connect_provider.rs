use aws_sdk_iam::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderOutput;
use aws_sdk_iam::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderOutput;
use aws_sdk_iam::operation::delete_open_id_connect_provider::DeleteOpenIdConnectProviderOutput;
use aws_sdk_iam::operation::get_open_id_connect_provider::GetOpenIdConnectProviderOutput;
use aws_sdk_iam::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsOutput;
use aws_sdk_iam::operation::list_open_id_connect_providers::ListOpenIdConnectProvidersOutput;
use aws_sdk_iam::operation::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderOutput;
use aws_sdk_iam::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderOutput;
use aws_sdk_iam::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderOutput;
use aws_sdk_iam::operation::update_open_id_connect_provider_thumbprint::UpdateOpenIdConnectProviderThumbprintOutput;
use aws_sdk_iam::types::OpenIdConnectProviderListEntry;
use aws_smithy_types::DateTime;
use chrono::Utc;
use sqlx::{Executor, Sqlite, Transaction};

use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::open_id_connect_provider::{
    InsertOpenIdConnectProvider, SelectOpenIdConnectProvider,
};
use crate::http::aws::iam::db::types::tags::ListTagsQuery;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::ActionError;
use crate::http::aws::iam::types::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::create_open_id_connect_provider::CreateOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::delete_open_id_connect_provider::DeleteOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::get_open_id_connect_provider::GetOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsRequest;
use crate::http::aws::iam::types::list_open_id_connect_providers::ListOpenIdConnectProvidersRequest;
use crate::http::aws::iam::types::remove_client_id_from_open_id_connect_provider::RemoveClientIdFromOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::tag_open_id_connect_provider::TagOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::untag_open_id_connect_provider::UntagOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::update_open_id_connect_provider_thumbprint::UpdateOpenIdConnectProviderThumbprintRequest;
use crate::http::aws::iam::{constants, db};

pub(crate) async fn find_id_by_arn<'a, E>(executor: E, account_id: i64, arn: &str) -> Result<i64, ActionError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::open_id_connect_provider::find_id_by_arn(executor, account_id, arn).await? {
        Some(provider_id) => Ok(provider_id),
        None => {
            return Err(ActionError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM OpenID connect provider with ARN '{}' doesn't exist.", arn).as_str(),
            ));
        }
    }
}

pub(crate) async fn find_by_arn<'a, E>(
    executor: E, account_id: i64, arn: &str,
) -> Result<SelectOpenIdConnectProvider, ActionError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::open_id_connect_provider::find_by_arn(executor, account_id, arn).await? {
        Some(provider_id) => Ok(provider_id),
        None => {
            return Err(ActionError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM OpenID connect provider with ARN '{}' doesn't exist.", arn).as_str(),
            ));
        }
    }
}

pub(crate) async fn add_client_id_to_open_id_connect_provider<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &AddClientIdToOpenIdConnectProviderRequest,
) -> Result<AddClientIdToOpenIdConnectProviderOutput, ActionError> {
    input.validate("$")?;
    let arn = input.open_id_connect_provider_arn().unwrap();
    let provider_id = find_id_by_arn(tx.as_mut(), ctx.account_id, arn).await?;
    db::open_id_connect_provider_client_id::create(tx, provider_id, input.client_id().unwrap()).await?;
    let output = AddClientIdToOpenIdConnectProviderOutput::builder().build();
    Ok(output)
}

pub(crate) async fn create_open_id_connect_provider<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &CreateOpenIdConnectProviderRequest,
) -> Result<CreateOpenIdConnectProviderOutput, ActionError> {
    input.validate("$")?;

    let current_time = Utc::now().timestamp();

    let arn = format!(
        "arn:aws:iam::{:0>12}:oidc-provider/{}",
        ctx.account_id,
        input
            .url()
            .unwrap()
            .strip_prefix(constants::open_id_connect_provider::URL_PREFIX)
            .unwrap()
    );

    let mut insert_provider = InsertOpenIdConnectProvider {
        id: None,
        account_id: ctx.account_id,
        arn,
        url: input.url().unwrap().to_owned(),
        create_date: current_time,
    };
    db::open_id_connect_provider::create(tx, &mut insert_provider).await?;
    let provider_id = insert_provider.id.unwrap();

    if let Some(client_id_list) = input.client_id_list() {
        db::open_id_connect_provider_client_id::create_all(tx, provider_id, client_id_list).await?;
    }

    if let Some(thumbprints) = input.thumbprint_list() {
        db::open_id_connect_provider_client_thumbprint::create_all(tx, provider_id, thumbprints).await?;
    }

    let mut tags = super::tag::prepare_for_db(input.tags(), provider_id);
    db::Tags::OpenIdConnectProvider.save_all(tx, &mut tags).await?;

    let output = CreateOpenIdConnectProviderOutput::builder()
        .open_id_connect_provider_arn(&insert_provider.arn)
        .set_tags(super::tag::prepare_for_output(&tags))
        .build();
    Ok(output)
}

pub(crate) async fn list_open_id_connect_provider_tags<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListOpenIdConnectProviderTagsRequest,
) -> Result<ListOpenIdConnectProviderTagsOutput, ActionError> {
    input.validate("$")?;

    let provider_id =
        find_id_by_arn(tx.as_mut(), ctx.account_id, input.open_id_connect_provider_arn().unwrap()).await?;

    let query = ListTagsQuery::new(input.max_items(), input.marker_type());
    let found_tags = db::Tags::OpenIdConnectProvider
        .list(tx.as_mut(), provider_id, &query)
        .await?;

    let tags = super::common::convert_and_limit(&found_tags, query.limit);
    let marker = super::common::create_encoded_marker(&query, found_tags.len())?;

    let output = ListOpenIdConnectProviderTagsOutput::builder()
        .set_tags(tags)
        .set_is_truncated(marker.as_ref().map(|_v| true))
        .set_marker(marker)
        .build()
        .unwrap();
    Ok(output)
}

pub(crate) async fn tag_open_id_connect_provider<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &TagOpenIdConnectProviderRequest,
) -> Result<TagOpenIdConnectProviderOutput, ActionError> {
    input.validate("$")?;

    let provider_id =
        find_id_by_arn(tx.as_mut(), ctx.account_id, input.open_id_connect_provider_arn().unwrap()).await?;
    let mut provider_tags = super::tag::prepare_for_db(input.tags(), provider_id);

    db::Tags::OpenIdConnectProvider.save_all(tx, &mut provider_tags).await?;
    let count = db::Tags::OpenIdConnectProvider.count(tx.as_mut(), provider_id).await?;
    if count > constants::tag::MAX_COUNT {
        return Err(ActionError::new(
            ApiErrorKind::LimitExceeded,
            format!("Cannot assign more than {} tags to IAM OpenID connect provider.", constants::tag::MAX_COUNT)
                .as_str(),
        ));
    }

    let output = TagOpenIdConnectProviderOutput::builder().build();
    Ok(output)
}

pub(crate) async fn untag_open_id_connect_provider<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &UntagOpenIdConnectProviderRequest,
) -> Result<UntagOpenIdConnectProviderOutput, ActionError> {
    input.validate("$")?;

    let provider_id =
        find_id_by_arn(tx.as_mut(), ctx.account_id, input.open_id_connect_provider_arn().unwrap().trim()).await?;

    db::Tags::OpenIdConnectProvider
        .delete_all(tx, provider_id, &input.tag_keys())
        .await?;

    let output = UntagOpenIdConnectProviderOutput::builder().build();
    Ok(output)
}

pub(crate) async fn update_open_id_connect_provider_thumbprint<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &UpdateOpenIdConnectProviderThumbprintRequest,
) -> Result<UpdateOpenIdConnectProviderThumbprintOutput, ActionError> {
    input.validate("$")?;

    let output = UpdateOpenIdConnectProviderThumbprintOutput::builder().build();
    Ok(output)
}

pub(crate) async fn remove_client_id_from_open_id_connect_provider<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &RemoveClientIdFromOpenIdConnectProviderRequest,
) -> Result<RemoveClientIdFromOpenIdConnectProviderOutput, ActionError> {
    input.validate("$")?;

    let arn = input.open_id_connect_provider_arn().unwrap();
    let provider_id = find_id_by_arn(tx.as_mut(), ctx.account_id, arn).await?;

    let client_id = input.client_id().unwrap();
    let is_deleted = db::open_id_connect_provider_client_id::delete(tx.as_mut(), provider_id, client_id).await?;
    if !is_deleted {
        return Err(ActionError::new(ApiErrorKind::NoSuchEntity, "Entity does not exist"));
    }

    let output = RemoveClientIdFromOpenIdConnectProviderOutput::builder().build();
    Ok(output)
}

pub(crate) async fn delete_open_id_connect_provider<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &DeleteOpenIdConnectProviderRequest,
) -> Result<DeleteOpenIdConnectProviderOutput, ActionError> {
    input.validate("$")?;

    let output = DeleteOpenIdConnectProviderOutput::builder().build();
    Ok(output)
}

pub(crate) async fn get_open_id_connect_provider<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &GetOpenIdConnectProviderRequest,
) -> Result<GetOpenIdConnectProviderOutput, ActionError> {
    input.validate("$")?;

    let arn = input.open_id_connect_provider_arn().unwrap();
    let provider = find_by_arn(tx.as_mut(), ctx.account_id, arn).await?;
    let client_ids = db::open_id_connect_provider_client_id::list(tx.as_mut(), provider.id)
        .await?
        .into_iter()
        .map(|c| c.client_id)
        .collect();

    let thumbprints = db::open_id_connect_provider_client_thumbprint::list(tx.as_mut(), provider.id)
        .await?
        .into_iter()
        .map(|t| t.thumbprint)
        .collect();

    let output = GetOpenIdConnectProviderOutput::builder()
        .url(&provider.url)
        .create_date(DateTime::from_secs(provider.create_date))
        .set_client_id_list(Some(client_ids))
        .set_thumbprint_list(Some(thumbprints))
        .build();
    Ok(output)
}

pub(crate) async fn list_open_id_connect_providers<'a>(
    tx: &mut Transaction<'a, Sqlite>, ctx: &OperationCtx, input: &ListOpenIdConnectProvidersRequest,
) -> Result<ListOpenIdConnectProvidersOutput, ActionError> {
    input.validate("$")?;

    let mut providers = vec![];
    let found_providers = db::open_id_connect_provider::list(tx.as_mut(), ctx.account_id).await?;
    for found_provider in found_providers {
        let provider = OpenIdConnectProviderListEntry::builder()
            .arn(&found_provider.arn)
            .build();
        providers.push(provider);
    }

    let output = ListOpenIdConnectProvidersOutput::builder()
        .set_open_id_connect_provider_list(Some(providers))
        .build();
    Ok(output)
}
