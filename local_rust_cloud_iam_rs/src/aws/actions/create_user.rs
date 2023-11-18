use std::fmt::Error;

use aws_sdk_iam::{
    operation::create_user::{CreateUserInput, CreateUserOutput},
    types::{Tag, User},
};
use aws_smithy_xml::encode::XmlWriter;
use local_rust_cloud_sqlite::Database;

use super::{action::Iam, constants::XMLNS, query::QueryReader, OutputWrapper};

const PROPERTY_USERNAME: &str = "UserName";
const PROPERTY_PATH: &str = "Path";
const PROPERTY_PERMISSIONS_BOUNDARY: &str = "PermissionsBoundary";

pub type IamCreateUserOutput = OutputWrapper<CreateUserOutput>;

impl Iam {
    pub async fn create_user<'a, I: Into<CreateUserInput>>(
        db: &Database, account_id: i64, request_id: impl Into<String>, input: I,
    ) -> Result<IamCreateUserOutput, Error> {
        let input = input.into();
        let user = User::builder().build();
        let result = CreateUserOutput::builder().user(user).build();
        Result::Ok(OutputWrapper::new(result, request_id.into()))
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
                input_tags.push(Tag::builder().key(tag.key).value(tag.value.unwrap_or("".to_string())).build());
            }
            builder.set_tags(Option::Some(input_tags)).build().unwrap()
        }
    }
}

impl From<IamCreateUserOutput> for String {
    fn from(val: IamCreateUserOutput) -> Self {
        let mut out = String::new();
        let mut doc = XmlWriter::new(&mut out);

        let mut create_user_response_tag = doc.start_el("CreateUserResponse").write_ns(XMLNS, None).finish();

        let mut create_user_result_tag = create_user_response_tag.start_el("CreateUserResult").finish();

        if val.inner.user().is_some() {
            let user = val.inner.user().unwrap();
            let mut user_tag = create_user_result_tag.start_el("User").finish();

            if user.tags().is_some() {
                let mut tags_tag = user_tag.start_el("Tags").finish();
                let tags = user.tags().unwrap();
                for tag in tags {
                    let mut tag_tag = tags_tag.start_el("Tag.member.0").finish();
                    local_rust_cloud_xml::write_tag_with_value(&mut tag_tag, "Key", tag.key());
                    local_rust_cloud_xml::write_tag_with_value(&mut tag_tag, "Value", tag.value());
                    tag_tag.finish();
                }
                tags_tag.finish();
            }

            user_tag.finish();
        }

        create_user_result_tag.finish();

        let mut response_metadata_tag = create_user_response_tag.start_el("ResponseMetadata").finish();
        local_rust_cloud_xml::write_tag_with_value(&mut response_metadata_tag, "RequestId", Option::Some(val.request_id));
        response_metadata_tag.finish();

        create_user_response_tag.finish();
        return out;
    }
}
