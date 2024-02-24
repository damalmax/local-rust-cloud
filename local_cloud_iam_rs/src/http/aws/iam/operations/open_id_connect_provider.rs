use aws_sdk_iam::operation::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderOutput;
use aws_sdk_iam::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderOutput;
use aws_sdk_iam::operation::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsOutput;
use aws_sdk_iam::operation::tag_open_id_connect_provider::TagOpenIdConnectProviderOutput;
use aws_sdk_iam::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderOutput;
use chrono::Utc;
use sqlx::{Executor, Sqlite};

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::open_id_connect_provider::InsertOpenIdConnectProvider;
use crate::http::aws::iam::db::types::tags::ListTagsQuery;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::add_client_id_to_open_id_connect_provider::AddClientIdToOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::create_open_id_connect_provider::CreateOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::list_open_id_connect_provider_tags::ListOpenIdConnectProviderTagsRequest;
use crate::http::aws::iam::types::tag_open_id_connect_provider::TagOpenIdConnectProviderRequest;
use crate::http::aws::iam::types::untag_open_id_connect_provider::UntagOpenIdConnectProviderRequest;
use crate::http::aws::iam::{constants, db};

pub(crate) async fn add_client_id_to_open_id_connect_provider(
    ctx: &OperationCtx, input: &AddClientIdToOpenIdConnectProviderRequest, db: &LocalDb,
) -> Result<AddClientIdToOpenIdConnectProviderOutput, OperationError> {
    input.validate("$")?;
    let mut tx = db.new_tx().await?;
    let arn = input.open_id_connect_provider_arn().unwrap();
    let provider_id = find_id_by_arn(tx.as_mut(), ctx.account_id, arn).await?;
    db::open_id_connect_provider_client_id::create(&mut tx, provider_id, input.client_id().unwrap()).await?;
    let output = AddClientIdToOpenIdConnectProviderOutput::builder().build();
    tx.commit().await?;
    Ok(output)
}

pub(crate) async fn find_id_by_arn<'a, E>(executor: E, account_id: i64, arn: &str) -> Result<i64, OperationError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::open_id_connect_provider::find_id_by_arn(executor, account_id, arn).await? {
        Some(provider_id) => Ok(provider_id),
        None => {
            return Err(OperationError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM OpenID connect provider with ARN '{}' doesn't exist.", arn).as_str(),
            ));
        }
    }
}

pub(crate) async fn create_open_id_connect_provider(
    ctx: &OperationCtx, input: &CreateOpenIdConnectProviderRequest, db: &LocalDb,
) -> Result<CreateOpenIdConnectProviderOutput, OperationError> {
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

    let mut tx = db.new_tx().await?;

    let mut insert_provider = InsertOpenIdConnectProvider {
        id: None,
        account_id: ctx.account_id,
        arn,
        url: input.url().unwrap().to_owned(),
        create_date: current_time,
    };
    db::open_id_connect_provider::create(&mut tx, &mut insert_provider).await?;
    let provider_id = insert_provider.id.unwrap();

    if let Some(client_id_list) = input.client_id_list() {
        db::open_id_connect_provider_client_id::create_all(&mut tx, provider_id, client_id_list).await?;
    }

    if let Some(thumbprints) = input.thumbprint_list() {
        db::open_id_connect_provider_client_thumbprint::create_all(&mut tx, provider_id, thumbprints).await?;
    }

    let mut tags = super::tag::prepare_for_db(input.tags(), provider_id);
    db::Tags::OpenIdConnectProvider.save_all(&mut tx, &mut tags).await?;

    let output = CreateOpenIdConnectProviderOutput::builder()
        .open_id_connect_provider_arn(&insert_provider.arn)
        .set_tags(super::tag::prepare_for_output(&tags))
        .build();

    tx.commit().await?;
    Ok(output)
}

pub(crate) async fn list_open_id_connect_provider_tags(
    ctx: &OperationCtx, input: &ListOpenIdConnectProviderTagsRequest, db: &LocalDb,
) -> Result<ListOpenIdConnectProviderTagsOutput, OperationError> {
    input.validate("$")?;

    let mut connection = db.new_connection().await?;

    let provider_id =
        find_id_by_arn(connection.as_mut(), ctx.account_id, input.open_id_connect_provider_arn().unwrap()).await?;

    let query = ListTagsQuery::new(input.max_items(), input.marker_type());
    let found_tags = db::Tags::OpenIdConnectProvider
        .list(connection.as_mut(), provider_id, &query)
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

pub(crate) async fn tag_open_id_connect_provider(
    ctx: &OperationCtx, input: &TagOpenIdConnectProviderRequest, db: &LocalDb,
) -> Result<TagOpenIdConnectProviderOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let provider_id =
        find_id_by_arn(tx.as_mut(), ctx.account_id, input.open_id_connect_provider_arn().unwrap()).await?;
    let mut provider_tags = super::tag::prepare_for_db(input.tags(), provider_id);

    db::Tags::OpenIdConnectProvider
        .save_all(&mut tx, &mut provider_tags)
        .await?;
    let count = db::Tags::OpenIdConnectProvider.count(tx.as_mut(), provider_id).await?;
    if count > constants::tag::MAX_COUNT {
        return Err(OperationError::new(
            ApiErrorKind::LimitExceeded,
            format!("Cannot assign more than {} tags to IAM OpenID connect provider.", constants::tag::MAX_COUNT)
                .as_str(),
        ));
    }

    let output = TagOpenIdConnectProviderOutput::builder().build();

    tx.commit().await?;

    Ok(output)
}

pub(crate) async fn untag_open_id_connect_provider(
    ctx: &OperationCtx, input: &UntagOpenIdConnectProviderRequest, db: &LocalDb,
) -> Result<UntagOpenIdConnectProviderOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let provider_id =
        find_id_by_arn(tx.as_mut(), ctx.account_id, input.open_id_connect_provider_arn().unwrap().trim()).await?;

    db::Tags::OpenIdConnectProvider
        .delete_all(&mut tx, provider_id, &input.tag_keys())
        .await?;

    let output = UntagOpenIdConnectProviderOutput::builder().build();

    tx.commit().await?;

    Ok(output)
}
