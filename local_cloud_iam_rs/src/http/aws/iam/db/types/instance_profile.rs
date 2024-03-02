use aws_sdk_iam::types::InstanceProfile;
use aws_smithy_types::DateTime;
use sqlx::FromRow;

use crate::http::aws::iam::db::types::common::Pageable;
use crate::http::aws::iam::types::list_instance_profiles::ListInstanceProfilesRequest;

#[derive(Debug)]
pub(crate) struct InsertInstanceProfile {
    pub(crate) id: Option<i64>,
    pub(crate) account_id: i64,
    pub(crate) instance_profile_name: String,
    pub(crate) instance_profile_id: String,
    pub(crate) arn: String,
    pub(crate) path: String,
    pub(crate) create_date: i64,
}

#[derive(Debug, FromRow)]
pub(crate) struct SelectInstanceProfile {
    pub(crate) id: i64,
    pub(crate) instance_profile_name: String,
    pub(crate) instance_profile_id: String,
    pub(crate) arn: String,
    pub(crate) path: String,
    pub(crate) create_date: i64,
}

#[derive(Debug, FromRow)]
pub(crate) struct SelectRoleForInstanceProfile {
    pub(crate) id: i64,
    pub(crate) role_name: String,
    pub(crate) assume_role_policy_document: String,
    pub(crate) arn: String,
    pub(crate) path: String,
    pub(crate) role_id: String,
    pub(crate) create_date: i64,
}

#[derive(Debug)]
pub(crate) struct ListInstanceProfilesQuery {
    pub(crate) path_prefix: String,
    pub(crate) limit: i32,
    pub(crate) skip: i32,
}

impl Pageable for &ListInstanceProfilesQuery {
    fn limit(&self) -> i32 {
        self.limit
    }

    fn skip(&self) -> i32 {
        self.skip
    }
}

impl Into<ListInstanceProfilesQuery> for &ListInstanceProfilesRequest {
    fn into(self) -> ListInstanceProfilesQuery {
        let limit = match self.max_items() {
            None => 10,
            Some(v) => *v,
        };

        let skip = match self.marker_type() {
            None => 0,
            // unwrap is safe since marker must be validated before DB query preparation
            Some(marker_type) => marker_type.marker().unwrap().truncate_amount,
        };

        ListInstanceProfilesQuery {
            path_prefix: self.path_prefix().unwrap_or("/").to_owned(),
            limit: if limit < 1 { 10 } else { limit },
            skip,
        }
    }
}

impl From<&SelectInstanceProfile> for InstanceProfile {
    fn from(value: &SelectInstanceProfile) -> Self {
        InstanceProfile::builder()
            .path(&value.path)
            .create_date(DateTime::from_secs(value.create_date))
            .instance_profile_id(&value.instance_profile_id)
            .instance_profile_name(&value.instance_profile_name)
            .arn(&value.arn)
            .build()
            .unwrap()
    }
}
