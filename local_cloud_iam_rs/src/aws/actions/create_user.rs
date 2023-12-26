use std::fmt::Error;
use std::time::SystemTime;

use aws_sdk_iam::{
    operation::create_user::{CreateUserInput, CreateUserOutput},
    types::{Tag, User},
};
use aws_smithy_types::DateTime;

use local_cloud_db::Database;

use super::{action::Iam, create_user_response::LocalCreateUserOutput, query::QueryReader, OutputWrapper};

const PROPERTY_USERNAME: &str = "UserName";
const PROPERTY_PATH: &str = "Path";
const PROPERTY_PERMISSIONS_BOUNDARY: &str = "PermissionsBoundary";

impl Iam {
    pub async fn create_user<'a, I: Into<CreateUserInput>>(
        db: &Database, account_id: i64, request_id: impl Into<String>, input: I,
    ) -> Result<LocalCreateUserOutput, Error> {
        let input = input.into();
        let user = User::builder()
            .path("/")
            .arn("")
            .user_name("name")
            .user_id("id")
            .create_date(DateTime::from(SystemTime::now()))
            .build()
            .unwrap();
        let result = CreateUserOutput::builder().user(user).build();
        Ok(OutputWrapper::new(result, request_id.into()))
    }
}

impl Into<CreateUserInput> for QueryReader {
    fn into(self) -> CreateUserInput {
        let builder = CreateUserInput::builder()
            .set_user_name(self.get_string(PROPERTY_USERNAME))
            .set_path(self.get_string(PROPERTY_PATH))
            .set_permissions_boundary(self.get_string(PROPERTY_PERMISSIONS_BOUNDARY));

        let tags = self.get_tags();
        if tags.is_none() {
            builder.set_tags(Option::None).build().unwrap()
        } else {
            let mut input_tags: Vec<Tag> = vec![];
            for tag in tags.unwrap() {
                input_tags.push(Tag::builder().key(tag.key()).value(tag.value()).build().unwrap());
            }
            builder.set_tags(Option::Some(input_tags)).build().unwrap()
        }
    }
}
