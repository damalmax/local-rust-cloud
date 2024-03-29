DROP INDEX IF EXISTS fk_server_certificate_tags__parent_id;
DROP TABLE IF EXISTS server_certificate_tags;

DROP INDEX IF EXISTS fk_server_certificates__account_id;
DROP TABLE IF EXISTS server_certificates;

DROP INDEX IF EXISTS fk_signing_certificates__account_id;
DROP TABLE IF EXISTS signing_certificates;

DROP INDEX IF EXISTS fk_user_ssh_public_keys__user_id;
DROP TABLE IF EXISTS user_ssh_public_keys;

DROP INDEX IF EXISTS fk_mfa_device_tags__parent_id;
DROP TABLE IF EXISTS mfa_device_tags;
DROP INDEX IF EXISTS fk_mfa_devices__account_id;
DROP TABLE IF EXISTS mfa_devices;

DROP INDEX IF EXISTS fk_saml_provider_tags__parent_id;
DROP TABLE IF EXISTS saml_provider_tags;

DROP INDEX IF EXISTS fk_saml_providers__account_id;
DROP TABLE IF EXISTS saml_providers;

DROP INDEX IF EXISTS fk_role_inline_policies__parent_id;
DROP TABLE IF EXISTS role_inline_policies;
DROP INDEX IF EXISTS fk_user_inline_policies__parent_id;
DROP TABLE IF EXISTS user_inline_policies;
DROP INDEX IF EXISTS fk_group_inline_policies__parent_id;
DROP TABLE IF EXISTS group_inline_policies;

DROP INDEX IF EXISTS fk_policy_users__user_id;
DROP INDEX IF EXISTS fk_policy_users__policy_id;
DROP TABLE IF EXISTS policy_users;

DROP INDEX IF EXISTS fk_policy_roles__role_id;
DROP INDEX IF EXISTS fk_policy_roles__policy_id;
DROP TABLE IF EXISTS policy_roles;

DROP INDEX IF EXISTS fk_policy_groups__policy_id;
DROP INDEX IF EXISTS fk_policy_groups__group_id;
DROP TABLE IF EXISTS policy_groups;

DROP INDEX IF EXISTS fk_group_users__user_id;
DROP INDEX IF EXISTS fk_group_users__group_id;
DROP TABLE IF EXISTS group_users;

DROP INDEX IF EXISTS fk_role_tags__parent_id;
DROP TABLE IF EXISTS role_tags;

DROP INDEX IF EXISTS idx_roles__arn;
DROP INDEX IF EXISTS fk_roles__account_id;
DROP TABLE IF EXISTS roles;

DROP INDEX IF EXISTS fk_policy_tags__parent_id;
DROP TABLE IF EXISTS policy_tags;

DROP TRIGGER IF EXISTS auto_increment_policy_version;
DROP INDEX IF EXISTS fk_policy_versions__policy_id;
DROP TABLE IF EXISTS policy_versions;

DROP INDEX IF EXISTS fk_user_tags__parent_id;
DROP TABLE IF EXISTS user_tags;

DROP INDEX IF EXISTS idx_users__arn;
DROP INDEX IF EXISTS fk_users__account_id;
DROP TABLE IF EXISTS users;

DROP INDEX IF EXISTS fk_policies__policy_type;
DROP INDEX IF EXISTS fk_policies__policy_id;
DROP INDEX IF EXISTS idx_policies__arn;
DROP TABLE IF EXISTS policies;

DROP INDEX IF EXISTS idx_groups__arn;
DROP TABLE IF EXISTS groups;

DROP TABLE IF EXISTS unique_identifiers;
DROP TABLE IF EXISTS regions;
DROP TABLE IF EXISTS accounts;

DROP INDEX IF EXISTS fk_login_profiles__user_id;
DROP TABLE IF EXISTS login_profiles;