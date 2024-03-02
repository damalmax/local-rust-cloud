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
        use sqlx::{Sqlite, Transaction};

        use local_cloud_axum::local::web::{AwsQueryBody, XmlResponse};
        use local_cloud_db::LocalDb;

        use crate::http::aws::iam::actions::action::Action;
        use crate::http::aws::iam::actions::error::ApiError;
        use crate::http::aws::iam::outputs::wrapper::OutputWrapper;
        use crate::http::aws::iam::operations::ctx::OperationCtx;
        use crate::http::aws::iam::operations::error::ActionError;

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


            let output: Result<XmlResponse, ActionError> = match aws_request {
                $(
                    $name::$variant(request) => handle_action_with_tx(&db, account_id, &aws_request_id, request).await,
                )+
            };
            let output: Result<XmlResponse, ApiError> = output.map_err(|error| match error {
                ActionError::Service { kind, msg } => {
                    tracing::error!("Failed to execute operation. Error message: {}", msg);
                    ApiError::new(kind, &msg, &aws_request_id)
                }
                ActionError::Validation(error) => ApiError::from_validation_error(&error, &aws_request_id)
            });
            match output {
                Ok(body) => {
                    Response::builder()
                                .header(CONTENT_TYPE_HEADER, CONTENT_TYPE_HEADER_VALUE)
                                .status(StatusCode::OK)
                                .body(body.to_owned())
                                .unwrap()
                }
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

        async fn handle_action_with_tx(db: &LocalDb, account_id: i64, aws_request_id: &str, action: impl Action) -> Result<XmlResponse, ActionError> {
            let mut tx = db.new_tx().await?;
            let response = action.execute(&mut tx, account_id, &aws_request_id)
                            .await
                            .map(|out| out.into())?;
            tx.commit().await?;
            Ok(response)
        }

        $(
            impl Action for $request {
                type Output = OutputWrapper<$response>;
                async fn execute<'a>(&self, tx: &mut Transaction<'a, Sqlite>, account_id: i64, aws_request_id: &str) -> Result<Self::Output, ActionError> {
                    let ctx = OperationCtx::new(account_id, aws_request_id);
                    let output = crate::http::aws::iam::operations::$resource::$action(tx, &ctx, self).await?;
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
        CreateAccessKey: access_key.create_access_key(CreateAccessKeyRequest) -> CreateAccessKeyOutput,
        CreateAccountAlias: account_alias.create_account_alias(CreateAccountAliasRequest) -> CreateAccountAliasOutput,
        CreateServiceLinkedRole: service_linked_role.create_service_linked_role(CreateServiceLinkedRoleRequest) -> CreateServiceLinkedRoleOutput,
        CreateServiceSpecificCredential: service_specific_credential.create_service_specific_credential(CreateServiceSpecificCredentialRequest) -> CreateServiceSpecificCredentialOutput,
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
        DeactivateMFADevice: mfa_device.deactivate_mfa_device(DeactivateMfaDeviceRequest) -> DeactivateMfaDeviceOutput,
        DeleteAccessKey: access_key.delete_access_key(DeleteAccessKeyRequest) -> DeleteAccessKeyOutput,
        DeleteAccountAlias: account_alias.delete_account_alias(DeleteAccountAliasRequest) -> DeleteAccountAliasOutput,
        DeleteAccountPasswordPolicy: account_password_policy.delete_account_password_policy(DeleteAccountPasswordPolicyRequest) -> DeleteAccountPasswordPolicyOutput,
        DeleteGroup: group.delete_group(DeleteGroupRequest) -> DeleteGroupOutput,
        DeleteGroupPolicy: group.delete_group_policy(DeleteGroupPolicyRequest) -> DeleteGroupPolicyOutput,
        DeleteInstanceProfile: instance_profile.delete_instance_profile(DeleteInstanceProfileRequest) -> DeleteInstanceProfileOutput,
        DeleteLoginProfile: login_profile.delete_login_profile(DeleteLoginProfileRequest) -> DeleteLoginProfileOutput,
        DeleteOpenIDConnectProvider: open_id_connect_provider.delete_open_id_connect_provider(DeleteOpenIdConnectProviderRequest) -> DeleteOpenIdConnectProviderOutput,
        DeletePolicy: policy.delete_policy(DeletePolicyRequest) -> DeletePolicyOutput,
        DeletePolicyVersion: policy.delete_policy_version(DeletePolicyVersionRequest) -> DeletePolicyVersionOutput,
        DeleteRole: role.delete_role(DeleteRoleRequest) -> DeleteRoleOutput,
        DeleteRolePermissionsBoundary: role.delete_role_permissions_boundary(DeleteRolePermissionsBoundaryRequest) -> DeleteRolePermissionsBoundaryOutput,
        DeleteRolePolicy: role.delete_role_policy(DeleteRolePolicyRequest) -> DeleteRolePolicyOutput,
        DeleteSAMLProvider: saml_provider.delete_saml_provider(DeleteSamlProviderRequest) -> DeleteSamlProviderOutput,
        DeleteServerCertificate: server_certificate.delete_server_certificate(DeleteServerCertificateRequest) -> DeleteServerCertificateOutput,
        DeleteServiceLinkedRole: service_linked_role.delete_service_linked_role(DeleteServiceLinkedRoleRequest) -> DeleteServiceLinkedRoleOutput,
        DeleteServiceSpecificCredential: service_specific_credential.delete_service_specific_credential(DeleteServiceSpecificCredentialRequest) -> DeleteServiceSpecificCredentialOutput,
        DeleteSigningCertificate: signing_certificate.delete_signing_certificate(DeleteSigningCertificateRequest) -> DeleteSigningCertificateOutput,
        DeleteSSHPublicKey: ssh_public_key.delete_ssh_public_key(DeleteSshPublicKeyRequest) -> DeleteSshPublicKeyOutput,
        DeleteUser: user.delete_user(DeleteUserRequest) -> DeleteUserOutput,
        DeleteUserPermissionsBoundary: user.delete_user_permissions_boundary(DeleteUserPermissionsBoundaryRequest) -> DeleteUserPermissionsBoundaryOutput,
        DeleteUserPolicy: user.delete_user_policy(DeleteUserPolicyRequest) -> DeleteUserPolicyOutput,
        DeleteVirtualMFADevice: mfa_device.delete_virtual_mfa_device(DeleteVirtualMfaDeviceRequest) -> DeleteVirtualMfaDeviceOutput,
        DetachGroupPolicy: group.detach_group_policy(DetachGroupPolicyRequest) -> DetachGroupPolicyOutput,
        DetachRolePolicy: role.detach_role_policy(DetachRolePolicyRequest) -> DetachRolePolicyOutput,
        DetachUserPolicy: user.detach_user_policy(DetachUserPolicyRequest) -> DetachUserPolicyOutput,
        EnableMFADevice: mfa_device.enable_mfa_device(EnableMfaDeviceRequest) -> EnableMfaDeviceOutput,
        GenerateCredentialReport: report.generate_credential_report(GenerateCredentialReportRequest) -> GenerateCredentialReportOutput,
        GenerateOrganizationsAccessReport: report.generate_organizations_access_report(GenerateOrganizationsAccessReportRequest) -> GenerateOrganizationsAccessReportOutput,
        GenerateServiceLastAccessedDetails: access_details.generate_service_last_accessed_details(GenerateServiceLastAccessedDetailsRequest) -> GenerateServiceLastAccessedDetailsOutput,
        GetAccessKeyLastUsed: access_details.get_access_key_last_used(GetAccessKeyLastUsedRequest) -> GetAccessKeyLastUsedOutput,
        GetAccountAuthorizationDetails: account.get_account_authorization_details(GetAccountAuthorizationDetailsRequest) -> GetAccountAuthorizationDetailsOutput,
        GetAccountPasswordPolicy: account.get_account_password_policy(GetAccountPasswordPolicyRequest) -> GetAccountPasswordPolicyOutput,
        GetAccountSummary: account.get_account_summary(GetAccountSummaryRequest) -> GetAccountSummaryOutput,
        GetContextKeysForCustomPolicy: policy.get_context_keys_for_custom_policy(GetContextKeysForCustomPolicyRequest) -> GetContextKeysForCustomPolicyOutput,
        GetContextKeysForPrincipalPolicy: policy.get_context_keys_for_principal_policy(GetContextKeysForPrincipalPolicyRequest)-> GetContextKeysForPrincipalPolicyOutput,
        GetCredentialReport: report.get_credential_report(GetCredentialReportRequest) -> GetCredentialReportOutput,
        GetGroup: group.get_group(GetGroupRequest) -> GetGroupOutput,
        GetGroupPolicy: group.get_group_policy(GetGroupPolicyRequest) -> GetGroupPolicyOutput,
        GetInstanceProfile: instance_profile.get_instance_profile(GetInstanceProfileRequest)-> GetInstanceProfileOutput,
        GetLoginProfile: login_profile.get_login_profile(GetLoginProfileRequest)-> GetLoginProfileOutput,
        GetMFADevice: mfa_device.get_mfa_device(GetMfaDeviceRequest) -> GetMfaDeviceOutput,
        GetOpenIDConnectProvider: open_id_connect_provider.get_open_id_connect_provider(GetOpenIdConnectProviderRequest) -> GetOpenIdConnectProviderOutput,
        GetOrganizationsAccessReport: report.get_organizations_access_report(GetOrganizationsAccessReportRequest)-> GetOrganizationsAccessReportOutput,
        GetPolicy: policy.get_policy(GetPolicyRequest) -> GetPolicyOutput,
        GetPolicyVersion: policy.get_policy_version(GetPolicyVersionRequest) -> GetPolicyVersionOutput,
        GetRole: role.get_role(GetRoleRequest) -> GetRoleOutput,
        GetRolePolicy: role.get_role_policy(GetRolePolicyRequest) -> GetRolePolicyOutput,
        GetSAMLProvider: saml_provider.get_saml_provider(GetSamlProviderRequest) -> GetSamlProviderOutput,
        GetServerCertificate: server_certificate.get_server_certificate(GetServerCertificateRequest) -> GetServerCertificateOutput,
        GetServiceLastAccessedDetails: service_last_accessed_details.get_service_last_accessed_details(GetServiceLastAccessedDetailsRequest) -> GetServiceLastAccessedDetailsOutput,
        GetServiceLastAccessedDetailsWithEntities: service_last_accessed_details.get_service_last_accessed_details_with_entities(GetServiceLastAccessedDetailsWithEntitiesRequest) -> GetServiceLastAccessedDetailsWithEntitiesOutput,
        GetServiceLinkedRoleDeletionStatus: service_linked_role.get_service_linked_role_deletion_status(GetServiceLinkedRoleDeletionStatusRequest) -> GetServiceLinkedRoleDeletionStatusOutput,
        GetSSHPublicKey: ssh_public_key.get_ssh_public_key(GetSshPublicKeyRequest) -> GetSshPublicKeyOutput,
        GetUser: user.get_user(GetUserRequest) -> GetUserOutput,
        GetUserPolicy: user.get_user_policy(GetUserPolicyRequest) -> GetUserPolicyOutput,
        ListAccessKeys: access_key.list_access_keys(ListAccessKeysRequest) -> ListAccessKeysOutput,
        ListAccountAliases: account_alias.list_account_aliases(ListAccountAliasesRequest) -> ListAccountAliasesOutput,
        ListAttachedGroupPolicies: group.list_attached_group_policies(ListAttachedGroupPoliciesRequest) -> ListAttachedGroupPoliciesOutput,
        ListAttachedRolePolicies: role.list_attached_role_policies(ListAttachedRolePoliciesRequest) -> ListAttachedRolePoliciesOutput,
        ListAttachedUserPolicies: user.list_attached_user_policies(ListAttachedUserPoliciesRequest) -> ListAttachedUserPoliciesOutput,
        ListEntitiesForPolicy: policy.list_entities_for_policy(ListEntitiesForPolicyRequest) -> ListEntitiesForPolicyOutput,
        ListGroupPolicies: group.list_group_policies(ListGroupPoliciesRequest) -> ListGroupPoliciesOutput,
        ListGroups: group.list_groups(ListGroupsRequest) -> ListGroupsOutput,
        ListGroupsForUser: group.list_groups_for_user(ListGroupsForUserRequest) -> ListGroupsForUserOutput,
        ListInstanceProfileTags: instance_profile.list_instance_profile_tags(ListInstanceProfileTagsRequest) -> ListInstanceProfileTagsOutput,
        ListInstanceProfiles: instance_profile.list_instance_profiles(ListInstanceProfilesRequest) -> ListInstanceProfilesOutput,
        ListInstanceProfilesForRole: instance_profile.list_instance_profiles_for_role(ListInstanceProfilesForRoleRequest) -> ListInstanceProfilesForRoleOutput,
        ListMFADeviceTags: mfa_device.list_mfa_device_tags(ListMfaDeviceTagsRequest) -> ListMfaDeviceTagsOutput,
        ListMFADevices: mfa_device.list_mfa_devices(ListMfaDevicesRequest) -> ListMfaDevicesOutput,
        ListOpenIDConnectProviderTags: open_id_connect_provider.list_open_id_connect_provider_tags(ListOpenIdConnectProviderTagsRequest) -> ListOpenIdConnectProviderTagsOutput,
        ListOpenIDConnectProviders: open_id_connect_provider.list_open_id_connect_providers(ListOpenIdConnectProvidersRequest) -> ListOpenIdConnectProvidersOutput,
        ListPolicies: policy.list_policies(ListPoliciesRequest) -> ListPoliciesOutput,
        ListPoliciesGrantingServiceAccess: policy.list_policies_granting_service_access(ListPoliciesGrantingServiceAccessRequest) -> ListPoliciesGrantingServiceAccessOutput,
        ListPolicyTags: policy.list_policy_tags(ListPolicyTagsRequest) -> ListPolicyTagsOutput,
        ListPolicyVersions: policy.list_policy_versions(ListPolicyVersionsRequest) -> ListPolicyVersionsOutput,
        ListRolePolicies: role.list_role_policies(ListRolePoliciesRequest) -> ListRolePoliciesOutput,
        ListRoleTags: role.list_role_tags(ListRoleTagsRequest) -> ListRoleTagsOutput,
        ListRoles: role.list_roles(ListRolesRequest) -> ListRolesOutput,
        ListSAMLProviderTags: saml_provider.list_saml_provider_tags(ListSamlProviderTagsRequest) -> ListSamlProviderTagsOutput,
        ListSAMLProviders: saml_provider.list_saml_providers(ListSamlProvidersRequest) -> ListSamlProvidersOutput,
        ListServerCertificateTags: server_certificate.list_server_certificate_tags(ListServerCertificateTagsRequest) -> ListServerCertificateTagsOutput,
        ListServerCertificates: server_certificate.list_server_certificates(ListServerCertificatesRequest) -> ListServerCertificatesOutput,
        ListServiceSpecificCredentials: service_specific_credential.list_service_specific_credentials(ListServiceSpecificCredentialsRequest) -> ListServiceSpecificCredentialsOutput,
        ListSigningCertificates: signing_certificate.list_signing_certificates(ListSigningCertificatesRequest) -> ListSigningCertificatesOutput,
        ListSSHPublicKeys: ssh_public_key.list_ssh_public_keys(ListSshPublicKeysRequest) -> ListSshPublicKeysOutput,
        ListUserPolicies: user.list_user_policies(ListUserPoliciesRequest) -> ListUserPoliciesOutput,
        ListUserTags: user.list_user_tags(ListUserTagsRequest) -> ListUserTagsOutput,
        ListUsers: user.list_users(ListUsersRequest) -> ListUsersOutput,
        ListVirtualMFADevices: mfa_device.list_virtual_mfa_devices(ListVirtualMfaDevicesRequest) -> ListVirtualMfaDevicesOutput,
        PutGroupPolicy: group.put_group_policy(PutGroupPolicyRequest) -> PutGroupPolicyOutput,
        PutRolePermissionsBoundary: role.put_role_permissions_boundary(PutRolePermissionsBoundaryRequest) -> PutRolePermissionsBoundaryOutput,
        PutRolePolicy: role.put_role_policy(PutRolePolicyRequest) -> PutRolePolicyOutput,
        PutUserPermissionsBoundary: user.put_user_permissions_boundary(PutUserPermissionsBoundaryRequest) -> PutUserPermissionsBoundaryOutput,
        PutUserPolicy: user.put_user_policy(PutUserPolicyRequest) -> PutUserPolicyOutput,
        RemoveClientIdFromOpenIDConnectProvider: open_id_connect_provider.remove_client_id_from_open_id_connect_provider(RemoveClientIdFromOpenIdConnectProviderRequest) -> RemoveClientIdFromOpenIdConnectProviderOutput,
        RemoveRoleFromInstanceProfile: instance_profile.remove_role_from_instance_profile(RemoveRoleFromInstanceProfileRequest) -> RemoveRoleFromInstanceProfileOutput,
        RemoveUserFromGroup: group.remove_user_from_group(RemoveUserFromGroupRequest) -> RemoveUserFromGroupOutput,
        ResetServiceSpecificCredential: service_specific_credential.reset_service_specific_credential(ResetServiceSpecificCredentialRequest) -> ResetServiceSpecificCredentialOutput,
        ResyncMFADevice: mfa_device.resync_mfa_device(ResyncMfaDeviceRequest) -> ResyncMfaDeviceOutput,
        SetDefaultPolicyVersion: policy.set_default_policy_version(SetDefaultPolicyVersionRequest) -> SetDefaultPolicyVersionOutput,
        SetSecurityTokenServicePreferences: security_token.set_security_token_service_preferences(SetSecurityTokenServicePreferencesRequest) -> SetSecurityTokenServicePreferencesOutput,
        SimulateCustomPolicy: simulate.simulate_custom_policy(SimulateCustomPolicyRequest) -> SimulateCustomPolicyOutput,
        SimulatePrincipalPolicy: simulate.simulate_principal_policy(SimulatePrincipalPolicyRequest) -> SimulatePrincipalPolicyOutput,
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
        UpdateAccessKey: access_key.update_access_key(UpdateAccessKeyRequest) -> UpdateAccessKeyOutput,
        UpdateAccountPasswordPolicy: account_password_policy.update_account_password_policy(UpdateAccountPasswordPolicyRequest) -> UpdateAccountPasswordPolicyOutput,
        UpdateAssumeRolePolicy: role.update_assume_role_policy(UpdateAssumeRolePolicyRequest) -> UpdateAssumeRolePolicyOutput,
        UpdateGroup: group.update_group(UpdateGroupRequest) -> UpdateGroupOutput,
        UpdateLoginProfile: login_profile.update_login_profile(UpdateLoginProfileRequest) -> UpdateLoginProfileOutput,
        UpdateOpenIDConnectProviderThumbprint: open_id_connect_provider.update_open_id_connect_provider_thumbprint(UpdateOpenIdConnectProviderThumbprintRequest) -> UpdateOpenIdConnectProviderThumbprintOutput,
        UpdateRole: role.update_role(UpdateRoleRequest) -> UpdateRoleOutput,
        UpdateRoleDescription: role.update_role_description(UpdateRoleDescriptionRequest) -> UpdateRoleDescriptionOutput,
        UpdateSAMLProvider: saml_provider.update_saml_provider(UpdateSamlProviderRequest) -> UpdateSamlProviderOutput,
        UpdateServerCertificate: server_certificate.update_server_certificate(UpdateServerCertificateRequest) -> UpdateServerCertificateOutput,
        UpdateServiceSpecificCredential: service_specific_credential.update_service_specific_credential(UpdateServiceSpecificCredentialRequest) -> UpdateServiceSpecificCredentialOutput,
        UpdateSigningCertificate: signing_certificate.update_signing_certificate(UpdateSigningCertificateRequest) -> UpdateSigningCertificateOutput,
        UpdateSSHPublicKey: ssh_public_key.update_ssh_public_key(UpdateSshPublicKeyRequest) -> UpdateSshPublicKeyOutput,
        UpdateUser: user.update_user(UpdateUserRequest) -> UpdateUserOutput,
        UploadServerCertificate: server_certificate.upload_server_certificate(UploadServerCertificateRequest) -> UploadServerCertificateOutput,
        UploadSigningCertificate: signing_certificate.upload_signing_certificate(UploadSigningCertificateRequest) -> UploadSigningCertificateOutput,
        UploadSSHPublicKey: ssh_public_key.upload_ssh_public_key(UploadSshPublicKeyRequest) -> UploadSshPublicKeyOutput,
    }
}
