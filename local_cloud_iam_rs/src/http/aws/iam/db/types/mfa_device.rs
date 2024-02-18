use aws_sdk_iam::types::{User, VirtualMfaDevice};
use aws_smithy_types::DateTime;
use sqlx::FromRow;

use crate::http::aws::iam::db::types::common::Pageable;
use crate::http::aws::iam::types::assignment_status_type::AssignmentStatusType;
use crate::http::aws::iam::types::list_virtual_mfa_devices_request::ListVirtualMfaDevicesRequest;

#[derive(Debug)]
pub(crate) struct InsertMfaDevice {
    pub(crate) id: Option<i64>,
    pub(crate) account_id: i64,
    pub(crate) serial_number: String,
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) seed: Vec<u8>,
    pub(crate) create_date: i64,
}

#[derive(Debug, FromRow)]
pub(crate) struct SelectMfaDevice {
    pub(crate) id: i64,
    pub(crate) account_id: i64,
    pub(crate) serial_number: String,
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) seed: Vec<u8>,
    pub(crate) create_date: i64,
    pub(crate) enable_date: Option<i64>,
    pub(crate) user_id: Option<i64>,
    pub(crate) user_user_id: Option<String>,
    pub(crate) user_name: Option<String>,
    pub(crate) user_arn: Option<String>,
    pub(crate) user_path: Option<String>,
    pub(crate) user_create_date: Option<i64>,
    pub(crate) user_password_last_used: Option<i64>,
}

#[derive(Debug)]
pub(crate) struct EnableMfaDeviceQuery {
    pub(crate) id: i64,
    pub(crate) enable_date: i64,
    pub(crate) user_id: i64,
    pub(crate) code1: String,
    pub(crate) code2: String,
}

#[derive(Debug)]
pub(crate) struct ListVirtualMfaDevicesQuery {
    pub(crate) assignment_status: AssignmentStatusType,
    pub(crate) limit: i32,
    pub(crate) skip: i32,
}

impl Pageable for &ListVirtualMfaDevicesQuery {
    fn limit(&self) -> i32 {
        self.limit
    }

    fn skip(&self) -> i32 {
        self.skip
    }
}

impl Into<ListVirtualMfaDevicesQuery> for &ListVirtualMfaDevicesRequest {
    fn into(self) -> ListVirtualMfaDevicesQuery {
        let limit = match self.max_items() {
            None => 10,
            Some(v) => *v,
        };

        let skip = match self.marker_type() {
            None => 0,
            // unwrap is safe since marker must be validated before DB query preparation
            Some(marker_type) => marker_type.marker().unwrap().truncate_amount,
        };

        ListVirtualMfaDevicesQuery {
            assignment_status: self
                .assignment_status()
                .map(|status| status.clone())
                .unwrap_or(AssignmentStatusType::Any),
            limit: if limit < 1 { 10 } else { limit },
            skip,
        }
    }
}

impl From<&SelectMfaDevice> for VirtualMfaDevice {
    fn from(value: &SelectMfaDevice) -> Self {
        let user = match value.user_id {
            None => None,
            Some(_) => {
                let user = User::builder()
                    .set_user_name(value.user_name.as_ref().map(|s| s.to_owned()))
                    .set_arn(value.user_arn.as_ref().map(|s| s.to_owned()))
                    .set_path(value.user_path.as_ref().map(|s| s.to_owned()))
                    .set_create_date(value.user_create_date.map(|secs| DateTime::from_secs(secs)))
                    .set_user_id(value.user_user_id.as_ref().map(|s| s.to_owned()))
                    .set_password_last_used(value.user_password_last_used.map(|secs| DateTime::from_secs(secs)))
                    .build()
                    .unwrap();
                Some(user)
            }
        };

        VirtualMfaDevice::builder()
            .serial_number(&value.serial_number)
            .set_enable_date(value.enable_date.map(|secs| DateTime::from_secs(secs)))
            .set_user(user)
            .build()
            .unwrap()
    }
}
