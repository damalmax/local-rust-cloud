use aws_sdk_iam::operation::create_account_alias::CreateAccountAliasOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalCreateAccountAliasOutput = OutputWrapper<CreateAccountAliasOutput>;

impl From<LocalCreateAccountAliasOutput> for XmlResponse {
    fn from(val: LocalCreateAccountAliasOutput) -> Self {
        super::confirmation::xml_response("CreateAccountAliasResponse", &val.request_id)
    }
}
