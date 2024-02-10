pub(crate) mod action;
pub(crate) mod create_group;
pub(crate) mod create_instance_profile;
pub(crate) mod create_login_profile;
pub(crate) mod create_open_id_connect_provider;
pub(crate) mod create_policy;
pub(crate) mod create_policy_version;
pub(crate) mod create_role;
pub(crate) mod create_user;
pub(crate) mod error;

macro_rules! action {
    ($entity:ident, $operation:ident, $mod_request:ident, $request:ident, $response:ident) => {
        mod $operation {
            use aws_sdk_iam::operation::$operation::$response;

            use local_cloud_db::LocalDb;

            use crate::http::aws::iam;
            use crate::http::aws::iam::actions::action::Action;
            use crate::http::aws::iam::actions::error::ApiError;
            use crate::http::aws::iam::operations::ctx::OperationCtx;
            use crate::http::aws::iam::operations::error::OperationError;
            use crate::http::aws::iam::outputs::wrapper::OutputWrapper;
            use crate::http::aws::iam::types::$mod_request::$request;

            impl Action for $request {
                type Output = OutputWrapper<$response>;
                async fn execute(
                    &self, account_id: i64, aws_request_id: &str, db: &LocalDb,
                ) -> Result<Self::Output, ApiError> {
                    let ctx = OperationCtx::new(account_id, aws_request_id);
                    let output = iam::operations::$entity::$operation(&ctx, self, db)
                        .await
                        .map_err(|error| match error {
                            OperationError::Service { kind, msg } => ApiError::new(kind, &msg, aws_request_id),
                            OperationError::Validation(error) => {
                                ApiError::from_validation_error(&error, aws_request_id)
                            }
                        })?;

                    Ok(OutputWrapper::new(output, aws_request_id))
                }
            }
        }
    };
}

action!(
    instance_profile,
    add_role_to_instance_profile,
    add_role_to_instance_profile_request,
    AddRoleToInstanceProfileRequest,
    AddRoleToInstanceProfileOutput
);
action!(
    instance_profile,
    tag_instance_profile,
    tag_instance_profile_request,
    TagInstanceProfileRequest,
    TagInstanceProfileOutput
);
action!(
    instance_profile,
    list_instance_profile_tags,
    list_instance_profile_tags_request,
    ListInstanceProfileTagsRequest,
    ListInstanceProfileTagsOutput
);
action!(group, add_user_to_group, add_user_to_group_request, AddUserToGroupRequest, AddUserToGroupOutput);
action!(group, attach_group_policy, attach_group_policy_request, AttachGroupPolicyRequest, AttachGroupPolicyOutput);
action!(group, get_group, get_group_request, GetGroupRequest, GetGroupOutput);
action!(group, list_groups, list_groups_request, ListGroupsRequest, ListGroupsOutput);
action!(policy, list_policies, list_policies_request, ListPoliciesRequest, ListPoliciesOutput);
action!(policy, list_policy_tags, list_policy_tags_request, ListPolicyTagsRequest, ListPolicyTagsOutput);
action!(policy, tag_policy, tag_policy_request, TagPolicyRequest, TagPolicyOutput);
action!(user, attach_user_policy, attach_user_policy_request, AttachUserPolicyRequest, AttachUserPolicyOutput);
action!(user, list_users, list_users_request, ListUsersRequest, ListUsersOutput);
action!(user, list_user_tags, list_user_tags_request, ListUserTagsRequest, ListUserTagsOutput);
action!(user, tag_user, tag_user_request, TagUserRequest, TagUserOutput);
action!(login_profile, change_password, change_password_request, ChangePasswordRequest, ChangePasswordOutput);
action!(role, attach_role_policy, attach_role_policy_request, AttachRolePolicyRequest, AttachRolePolicyOutput);
action!(role, list_role_tags, list_role_tags_request, ListRoleTagsRequest, ListRoleTagsOutput);
action!(role, list_roles, list_roles_request, ListRolesRequest, ListRolesOutput);
action!(role, tag_role, tag_role_request, TagRoleRequest, TagRoleOutput);
