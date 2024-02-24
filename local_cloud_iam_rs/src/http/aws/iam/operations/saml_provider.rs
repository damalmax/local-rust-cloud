use aws_sdk_iam::operation::create_saml_provider::CreateSamlProviderOutput;
use aws_sdk_iam::operation::list_saml_provider_tags::ListSamlProviderTagsOutput;
use aws_sdk_iam::operation::tag_saml_provider::TagSamlProviderOutput;
use aws_sdk_iam::operation::untag_saml_provider::UntagSamlProviderOutput;
use chrono::Utc;
use sqlx::{Executor, Sqlite};

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::db::types::saml_provider::InsertSamlProvider;
use crate::http::aws::iam::db::types::tags::ListTagsQuery;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::create_saml_provider::CreateSamlProviderRequest;
use crate::http::aws::iam::types::list_saml_provider_tags::ListSamlProviderTagsRequest;
use crate::http::aws::iam::types::tag_saml_provider::TagSamlProviderRequest;
use crate::http::aws::iam::types::untag_saml_provider::UntagSamlProviderRequest;
use crate::http::aws::iam::{constants, db};

pub(crate) async fn find_id_by_arn<'a, E>(executor: E, account_id: i64, arn: &str) -> Result<i64, OperationError>
where
    E: 'a + Executor<'a, Database = Sqlite>,
{
    match db::saml_provider::find_id_by_arn(executor, account_id, arn).await? {
        Some(provider_id) => Ok(provider_id),
        None => {
            return Err(OperationError::new(
                ApiErrorKind::NoSuchEntity,
                format!("IAM SAML provider with ARN '{}' doesn't exist.", arn).as_str(),
            ));
        }
    }
}

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

    let mut saml_provider_tags = super::tag::prepare_for_db(input.tags(), insert_saml_provider.id.unwrap());

    db::Tags::SamlProvider
        .save_all(&mut tx, &mut saml_provider_tags)
        .await?;

    let output = CreateSamlProviderOutput::builder()
        .saml_provider_arn(insert_saml_provider.arn)
        .set_tags(super::tag::prepare_for_output(&saml_provider_tags))
        .build();

    tx.commit().await?;
    Ok(output)
}

pub(crate) async fn tag_saml_provider(
    ctx: &OperationCtx, input: &TagSamlProviderRequest, db: &LocalDb,
) -> Result<TagSamlProviderOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let saml_provider_id = find_id_by_arn(tx.as_mut(), ctx.account_id, input.saml_provider_arn().unwrap()).await?;
    let mut saml_provider_tags = super::tag::prepare_for_db(input.tags(), saml_provider_id);

    db::Tags::SamlProvider
        .save_all(&mut tx, &mut saml_provider_tags)
        .await?;
    let count = db::Tags::SamlProvider.count(tx.as_mut(), saml_provider_id).await?;
    if count > constants::tag::MAX_COUNT {
        return Err(OperationError::new(
            ApiErrorKind::LimitExceeded,
            format!("Cannot assign more than {} tags to IAM SAML provider.", constants::tag::MAX_COUNT).as_str(),
        ));
    }

    let output = TagSamlProviderOutput::builder().build();

    tx.commit().await?;

    Ok(output)
}

pub(crate) async fn list_saml_provider_tags(
    ctx: &OperationCtx, input: &ListSamlProviderTagsRequest, db: &LocalDb,
) -> Result<ListSamlProviderTagsOutput, OperationError> {
    input.validate("$")?;

    let mut connection = db.new_connection().await?;

    let provider_id = find_id_by_arn(connection.as_mut(), ctx.account_id, input.saml_provider_arn().unwrap()).await?;

    let query = ListTagsQuery::new(input.max_items(), input.marker_type());
    let found_tags = db::Tags::SamlProvider
        .list(connection.as_mut(), provider_id, &query)
        .await?;

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

pub(crate) async fn untag_saml_provider(
    ctx: &OperationCtx, input: &UntagSamlProviderRequest, db: &LocalDb,
) -> Result<UntagSamlProviderOutput, OperationError> {
    input.validate("$")?;

    let mut tx = db.new_tx().await?;

    let saml_provider_id =
        find_id_by_arn(tx.as_mut(), ctx.account_id, input.saml_provider_arn().unwrap().trim()).await?;

    db::Tags::SamlProvider
        .delete_all(&mut tx, saml_provider_id, &input.tag_keys())
        .await?;

    let output = UntagSamlProviderOutput::builder().build();

    tx.commit().await?;

    Ok(output)
}
