use aws_sdk_iam::operation::create_open_id_connect_provider::CreateOpenIdConnectProviderOutput;
use chrono::Utc;

use local_cloud_db::LocalDb;
use local_cloud_validate::NamedValidator;

use crate::http::aws::iam::db::types::open_id_connect_provider::InsertOpenIdConnectProvider;
use crate::http::aws::iam::operations::ctx::OperationCtx;
use crate::http::aws::iam::operations::error::OperationError;
use crate::http::aws::iam::types::create_open_id_connect_provider_request::CreateOpenIdConnectProviderRequest;
use crate::http::aws::iam::{constants, db};

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

    let mut tags = super::common::prepare_tags_for_insert(input.tags(), provider_id);
    db::open_id_connect_provider_tag::save_all(&mut tx, &mut tags).await?;

    let output = CreateOpenIdConnectProviderOutput::builder()
        .open_id_connect_provider_arn(&insert_provider.arn)
        .set_tags(super::common::prepare_tags_for_output(&tags))
        .build();

    tx.commit().await?;
    Ok(output)
}
