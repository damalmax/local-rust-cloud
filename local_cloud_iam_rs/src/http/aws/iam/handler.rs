macro_rules! action_handler {
    (
        $(#[$m:meta])*
        enum $name:ident {
            $($variant:ident: $resource:ident.$action:ident($request:ident) -> $response:ident),+
            $(,)?
        }
    ) => {
        use axum::response::Response;
        use axum::extract::State;
        use axum::http::StatusCode;
        use uuid::Uuid;

        use local_cloud_axum::local::web::{AwsQueryBody, XmlResponse};
        use local_cloud_db::LocalDb;

        use crate::http::aws::iam::actions::action::Action;
        use crate::http::aws::iam::actions::error::ApiError;
        use crate::http::aws::iam::outputs::wrapper::OutputWrapper;
        use crate::http::aws::iam::operations::ctx::OperationCtx;
        use crate::http::aws::iam::operations::error::OperationError;

        $(
            use aws_sdk_iam::operation::$action::$response;
            use crate::http::aws::iam::types::$action::$request;
        )+

        $(#[$m])*
        #[non_exhaustive]
        pub(crate) enum $name {
            $(
                $variant($request)
            ),+
        }

        const CONTENT_TYPE_HEADER: &str = "Content-Type";
        const CONTENT_TYPE_HEADER_VALUE: &str = "text/xml; charset=utf-8";

        pub(crate) async fn handle(State(db): State<LocalDb>, aws_query: AwsQueryBody<AwsRequest>,) -> Response<String> {
            let account_id = 1i64;
            let aws_request = aws_query.into_inner();
            let aws_request_id = Uuid::new_v4().to_string();
            let output: Result<XmlResponse, ApiError> = match aws_request {
                $(
                    $name::$variant(request) => request.execute(account_id, &aws_request_id, &db)
                        .await
                        .map(|out| out.into()),
                )+
            };
            match output {
                Ok(body) => Response::builder()
                                .header(CONTENT_TYPE_HEADER, CONTENT_TYPE_HEADER_VALUE)
                                .status(StatusCode::OK)
                                .body(body.to_owned())
                                .unwrap(),
                Err(err) => {
                    let error_code = err.kind.status_code();
                    let body: XmlResponse = err.into();
                    Response::builder()
                        .header(CONTENT_TYPE_HEADER, CONTENT_TYPE_HEADER_VALUE)
                        .status(error_code)
                        .body(body.to_owned())
                        .unwrap()
                }
            }
        }

        $(
            impl Action for $request {
                type Output = OutputWrapper<$response>;
                async fn execute(&self, account_id: i64, aws_request_id: &str, db: &LocalDb) -> Result<Self::Output, ApiError> {
                    let ctx = OperationCtx::new(account_id, aws_request_id);
                    let output = crate::http::aws::iam::operations::$resource::$action(&ctx, self, db)
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
        )+
    };
}

action_handler! {
    #[derive(serde::Deserialize, Debug)]
    #[serde(tag = "Action")]
    enum AwsRequest {
        AddClientIDToOpenIDConnectProvider: open_id_connect_provider.add_client_id_to_open_id_connect_provider(AddClientIdToOpenIdConnectProviderRequest) -> AddClientIdToOpenIdConnectProviderOutput,
        AddRoleToInstanceProfile: instance_profile.add_role_to_instance_profile(AddRoleToInstanceProfileRequest) -> AddRoleToInstanceProfileOutput,
        AddUserToGroup: group.add_user_to_group(AddUserToGroupRequest) -> AddUserToGroupOutput,
        AttachGroupPolicy: group.attach_group_policy(AttachGroupPolicyRequest) -> AttachGroupPolicyOutput,
        AttachRolePolicy: role.attach_role_policy(AttachRolePolicyRequest) -> AttachRolePolicyOutput,
        AttachUserPolicy: user.attach_user_policy(AttachUserPolicyRequest) -> AttachUserPolicyOutput,
        ChangePassword: login_profile.change_password(ChangePasswordRequest) -> ChangePasswordOutput,
        CreateGroup: group.create_group(CreateGroupRequest) -> CreateGroupOutput,
        CreateInstanceProfile: instance_profile.create_instance_profile(CreateInstanceProfileRequest) -> CreateInstanceProfileOutput,
        CreateLoginProfile: login_profile.create_login_profile(CreateLoginProfileRequest) -> CreateLoginProfileOutput,
        CreateOpenIDConnectProvider: open_id_connect_provider.create_open_id_connect_provider(CreateOpenIdConnectProviderRequest) -> CreateOpenIdConnectProviderOutput,
        CreatePolicy: policy.create_policy(CreatePolicyRequest) -> CreatePolicyOutput,
        CreatePolicyVersion: policy.create_policy_version(CreatePolicyVersionRequest) -> CreatePolicyVersionOutput,
        CreateRole: role.create_role(CreateRoleRequest) -> CreateRoleOutput,
        CreateSAMLProvider: saml_provider.create_saml_provider(CreateSamlProviderRequest) -> CreateSamlProviderOutput,
        CreateUser: user.create_user(CreateUserRequest) -> CreateUserOutput,
        CreateVirtualMFADevice: mfa_device.create_virtual_mfa_device(CreateVirtualMfaDeviceRequest) -> CreateVirtualMfaDeviceOutput,
        EnableMFADevice: mfa_device.enable_mfa_device(EnableMfaDeviceRequest) -> EnableMfaDeviceOutput,
        GetGroup: group.get_group(GetGroupRequest) -> GetGroupOutput,
        GetGroupPolicy: group.get_group_policy(GetGroupPolicyRequest) -> GetGroupPolicyOutput,
        GetMFADevice: mfa_device.get_mfa_device(GetMfaDeviceRequest) -> GetMfaDeviceOutput,
        GetRolePolicy: role.get_role_policy(GetRolePolicyRequest) -> GetRolePolicyOutput,
        GetUserPolicy: user.get_user_policy(GetUserPolicyRequest) -> GetUserPolicyOutput,
        ListGroups: group.list_groups(ListGroupsRequest) -> ListGroupsOutput,
        ListGroupsForUser: group.list_groups_for_user(ListGroupsForUserRequest) -> ListGroupsForUserOutput,
        ListGroupPolicies: group.list_group_policies(ListGroupPoliciesRequest) -> ListGroupPoliciesOutput,
        ListInstanceProfileTags: instance_profile.list_instance_profile_tags(ListInstanceProfileTagsRequest) -> ListInstanceProfileTagsOutput,
        ListMFADeviceTags: mfa_device.list_mfa_device_tags(ListMfaDeviceTagsRequest) -> ListMfaDeviceTagsOutput,
        ListOpenIDConnectProviderTags: open_id_connect_provider.list_open_id_connect_provider_tags(ListOpenIdConnectProviderTagsRequest) -> ListOpenIdConnectProviderTagsOutput,
        ListPolicies: policy.list_policies(ListPoliciesRequest) -> ListPoliciesOutput,
        ListPolicyVersions: policy.list_policy_versions(ListPolicyVersionsRequest) -> ListPolicyVersionsOutput,
        ListPolicyTags: policy.list_policy_tags(ListPolicyTagsRequest) -> ListPolicyTagsOutput,
        ListRoles: role.list_roles(ListRolesRequest) -> ListRolesOutput,
        ListSAMLProviderTags: saml_provider.list_saml_provider_tags(ListSamlProviderTagsRequest) -> ListSamlProviderTagsOutput,
        ListServerCertificateTags: server_certificate.list_server_certificate_tags(ListServerCertificateTagsRequest) -> ListServerCertificateTagsOutput,
        ListRolePolicies: role.list_role_policies(ListRolePoliciesRequest) -> ListRolePoliciesOutput,
        ListRoleTags: role.list_role_tags(ListRoleTagsRequest) -> ListRoleTagsOutput,
        ListUserPolicies: user.list_user_policies(ListUserPoliciesRequest) -> ListUserPoliciesOutput,
        ListUserTags: user.list_user_tags(ListUserTagsRequest) -> ListUserTagsOutput,
        ListUsers: user.list_users(ListUsersRequest) -> ListUsersOutput,
        ListVirtualMFADevices: mfa_device.list_virtual_mfa_devices(ListVirtualMfaDevicesRequest) -> ListVirtualMfaDevicesOutput,
        PutGroupPolicy: group.put_group_policy(PutGroupPolicyRequest) -> PutGroupPolicyOutput,
        PutRolePolicy: role.put_role_policy(PutRolePolicyRequest) -> PutRolePolicyOutput,
        PutUserPolicy: user.put_user_policy(PutUserPolicyRequest) -> PutUserPolicyOutput,
        TagInstanceProfile: instance_profile.tag_instance_profile(TagInstanceProfileRequest) -> TagInstanceProfileOutput,
        TagMFADevice: mfa_device.tag_mfa_device(TagMfaDeviceRequest) -> TagMfaDeviceOutput,
        TagOpenIDConnectProvider: open_id_connect_provider.tag_open_id_connect_provider(TagOpenIdConnectProviderRequest) -> TagOpenIdConnectProviderOutput,
        TagPolicy: policy.tag_policy(TagPolicyRequest) -> TagPolicyOutput,
        TagRole: role.tag_role(TagRoleRequest) -> TagRoleOutput,
        TagSAMLProvider: saml_provider.tag_saml_provider(TagSamlProviderRequest) -> TagSamlProviderOutput,
        TagServerCertificate: server_certificate.tag_server_certificate(TagServerCertificateRequest) -> TagServerCertificateOutput,
        TagUser: user.tag_user(TagUserRequest) -> TagUserOutput,
        UntagInstanceProfile: instance_profile.untag_instance_profile(UntagInstanceProfileRequest) -> UntagInstanceProfileOutput,
        UntagMFADevice: mfa_device.untag_mfa_device(UntagMfaDeviceRequest) -> UntagMfaDeviceOutput,
        UntagOpenIDConnectProvider: open_id_connect_provider.untag_open_id_connect_provider(UntagOpenIdConnectProviderRequest) -> UntagOpenIdConnectProviderOutput,
        UntagPolicy: policy.untag_policy(UntagPolicyRequest) -> UntagPolicyOutput,
        UntagRole: role.untag_role(UntagRoleRequest) -> UntagRoleOutput,
        UntagSAMLProvider: saml_provider.untag_saml_provider(UntagSamlProviderRequest) -> UntagSamlProviderOutput,
        UntagServerCertificate: server_certificate.untag_server_certificate(UntagServerCertificateRequest) -> UntagServerCertificateOutput,
        UntagUser: user.untag_user(UntagUserRequest) -> UntagUserOutput,
        UpdateUser: user.update_user(UpdateUserRequest) -> UpdateUserOutput,
        UploadServerCertificate: server_certificate.upload_server_certificate(UploadServerCertificateRequest) -> UploadServerCertificateOutput,
        UploadSigningCertificate: signing_certificate.upload_signing_certificate(UploadSigningCertificateRequest) -> UploadSigningCertificateOutput,
        UploadSSHPublicKey: ssh_public_key.upload_ssh_public_key(UploadSshPublicKeyRequest) -> UploadSshPublicKeyOutput,
    }
}
