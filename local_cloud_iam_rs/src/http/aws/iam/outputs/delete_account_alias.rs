use aws_sdk_iam::operation::delete_account_alias::DeleteAccountAliasOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalDeleteAccountAliasOutput = OutputWrapper<DeleteAccountAliasOutput>;

impl From<LocalDeleteAccountAliasOutput> for XmlResponse {
    fn from(val: LocalDeleteAccountAliasOutput) -> Self {
        super::confirmation::xml_response("DeleteAccountAliasResponse", &val.request_id)
    }
}
