-- Account
CREATE TABLE IF NOT EXISTS accounts
(
    id    INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    alias VARCHAR2(63)                      NOT NULL
);
INSERT INTO accounts(alias)
VALUES ('Home Account');
-- Region
CREATE TABLE IF NOT EXISTS regions
(
    id     INTEGER PRIMARY KEY NOT NULL,
    region VARCHAR2(20)        NOT NULL,
    name   VARCHAR2(30)        NOT NULL,
    UNIQUE (region)
);
INSERT INTO regions(id, region, name)
VALUES (1, 'eu-local-1', 'EU Local'),
       (2, 'eu-central-1', 'Europe (Frankfurt)'),
       (3, 'eu-central-2', 'Europe (Zurich)'),
       (4, 'eu-west-1', 'Europe (Ireland)'),
       (5, 'eu-west-2', 'Europe (London)'),
       (6, 'eu-west-3', 'Europe (Paris)'),
       (7, 'eu-south-1', 'Europe (Milan)'),
       (8, 'eu-south-2', 'Europe (Spain)'),
       (9, 'eu-north-1', 'Europe (Stockholm)'),
       (10, 'us-local-1', 'US Local'),
       (11, 'us-east-1', 'US East (N. Virginia)'),
       (12, 'us-east-2', 'US East (Ohio)'),
       (13, 'us-west-1', 'US West (N. California)'),
       (14, 'us-west-2', 'US West (Oregon)'),
       (15, 'us-gov-east-1', 'AWS GovCloud (US-East)'),
       (16, 'us-gov-west-1', 'AWS GovCloud (US-West)'),
       (17, 'af-south-1', 'Africa (Cape Town)'),
       (18, 'ap-local-1', 'AP Local'),
       (19, 'ap-east-1', 'Asia Pacific (Hong Kong)'),
       (20, 'ap-south-1', 'Asia Pacific (Mumbai)'),
       (21, 'ap-south-2', 'Asia Pacific (Hyderabad)'),
       (22, 'ap-southeast-1', 'Asia Pacific (Singapore)'),
       (23, 'ap-southeast-2', 'Asia Pacific (Sydney)'),
       (24, 'ap-southeast-3', 'Asia Pacific (Jakarta)'),
       (25, 'ap-southeast-4', 'Asia Pacific (Melbourne)'),
       (26, 'ap-northeast-1', 'Asia Pacific (Tokyo)'),
       (27, 'ap-northeast-2', 'Asia Pacific (Seoul)'),
       (28, 'ap-northeast-3', 'Asia Pacific (Osaka)'),
       (29, 'ca-central-1', 'Canada (Central)'),
       (30, 'il-central-1', 'Israel (Tel Aviv)'),
       (31, 'me-south-1', 'Middle East (Bahrain)'),
       (32, 'me-central-1', 'Middle East (UAE)'),
       (33, 'sa-east-1', 'South America (São Paulo)');
CREATE TABLE IF NOT EXISTS unique_identifiers
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    unique_id     VARCHAR2(21)                      NOT NULL,
    resource_type INTEGER                           NOT NULL,
    UNIQUE (unique_id)
);
-- Group
CREATE TABLE IF NOT EXISTS groups
(
    id                INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id        INTEGER REFERENCES accounts (id)  NOT NULL,
    group_name        VARCHAR2(128)                     NOT NULL,
    unique_group_name VARCHAR2(128)                     NOT NULL,
    arn               VARCHAR2(2048)                    NOT NULL,
    path              VARCHAR2(512)                     NOT NULL,
    group_id          VARCHAR2(21)                      NOT NULL,
    create_date       INTEGER                           NOT NULL,
    UNIQUE (arn),
    UNIQUE (group_id),
    UNIQUE (account_id, unique_group_name)
);
CREATE INDEX IF NOT EXISTS idx_groups__arn ON groups (arn ASC);
CREATE TABLE IF NOT EXISTS group_inline_policies
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    parent_id          INTEGER REFERENCES groups (id)    NOT NULL,
    policy_name        VARCHAR2(128)                     NOT NULL,
    unique_policy_name VARCHAR2(128)                     NOT NULL,
    policy_document    VARCHAR2(131072)                  NOT NULL,
    UNIQUE (parent_id, unique_policy_name)
);
CREATE INDEX IF NOT EXISTS fk_group_inline_policies__parent_id ON group_inline_policies (parent_id ASC);
-- Policy
CREATE TABLE IF NOT EXISTS policies
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id         INTEGER REFERENCES accounts (id)  NOT NULL,
    policy_name        VARCHAR2(128)                     NOT NULL,
    unique_policy_name VARCHAR2(128)                     NOT NULL,
    policy_id          VARCHAR2(21)                      NOT NULL,
    policy_type        INTEGER                           NOT NULL,
    arn                VARCHAR2(2048)                    NOT NULL,
    path               VARCHAR2(512)                     NOT NULL,
    is_attachable      BOOLEAN,
    description        VARCHAR2(200),
    create_date        INTEGER                           NOT NULL,
    update_date        INTEGER                           NOT NULL,
    UNIQUE (arn),
    UNIQUE (policy_id),
    UNIQUE (account_id, unique_policy_name)
);
CREATE INDEX IF NOT EXISTS idx_policies__arn ON policies (arn ASC);
CREATE INDEX IF NOT EXISTS fk_policies__policy_id ON policies (policy_id ASC);
CREATE INDEX IF NOT EXISTS fk_policies__policy_type ON policies (policy_type ASC);
-- User
CREATE TABLE IF NOT EXISTS users
(
    id              INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id      INTEGER REFERENCES accounts (id),
    username        VARCHAR2(128)                     NOT NULL,
    unique_username VARCHAR2(128)                     NOT NULL,
    arn             VARCHAR2(2048)                    NOT NULL,
    path            VARCHAR2(512)                     NOT NULL,
    user_id         VARCHAR2(21)                      NOT NULL,
    policy_id       INTEGER REFERENCES policies (id),
    create_date     INTEGER                           NOT NULL,
    last_used_date  INTEGER,
    UNIQUE (arn),
    UNIQUE (user_id),
    UNIQUE (account_id, unique_username)
);
CREATE INDEX IF NOT EXISTS fk_users__account_id ON users (account_id ASC);
CREATE INDEX IF NOT EXISTS idx_users__arn ON users (arn ASC);
INSERT INTO users(account_id,
                  username,
                  unique_username,
                  arn,
                  path,
                  user_id,
                  create_date)
VALUES (1,
        'Root',
        'ROOT',
        '"arn:aws:iam::000000000001:user/Root"',
        '/',
        'AIDAHOMECLOUDROOT101A',
        1706219306);
CREATE TABLE IF NOT EXISTS user_inline_policies
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    parent_id          INTEGER REFERENCES users (id)     NOT NULL,
    policy_name        VARCHAR2(128)                     NOT NULL,
    unique_policy_name VARCHAR2(128)                     NOT NULL,
    policy_document    VARCHAR2(131072)                  NOT NULL,
    UNIQUE (parent_id, unique_policy_name)
);
CREATE INDEX IF NOT EXISTS fk_user_inline_policies__parent_id ON user_inline_policies (parent_id ASC);
CREATE TABLE IF NOT EXISTS user_tags
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    parent_id INTEGER REFERENCES users (id)     NOT NULL,
    key       VARCHAR2(128)                     NOT NULL,
    value     VARCHAR2(256)                     NOT NULL,
    UNIQUE (parent_id, key)
);
CREATE INDEX IF NOT EXISTS fk_user_tags__parent_id ON user_tags (parent_id ASC);
CREATE TABLE IF NOT EXISTS policy_versions
(
    id                INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id        INTEGER REFERENCES accounts (id)  NOT NULL,
    policy_id         INTEGER REFERENCES policies (id)  NOT NULL,
    policy_version_id VARCHAR2(21)                      NOT NULL,
    policy_document   VARCHAR2(6144)                    NOT NULL,
    version           INTEGER, -- there in no constraint on the column since it will be auto-populated by trigger
    create_date       INTEGER                           NOT NULL,
    is_default        BOOLEAN                           NOT NULL
);
CREATE INDEX IF NOT EXISTS fk_policy_versions__policy_id ON policy_versions (policy_id ASC);
CREATE TRIGGER IF NOT EXISTS auto_increment_policy_version
    AFTER
        INSERT
    ON policy_versions
    WHEN new.version IS NULL
BEGIN
    UPDATE policy_versions
    SET version = (SELECT IFNULL(MAX(version), 0) + 1
                   FROM policy_versions
                   WHERE account_id = new.account_id
                     AND policy_id = new.policy_id)
    WHERE id = new.id;
END;
CREATE TABLE IF NOT EXISTS policy_tags
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    parent_id INTEGER REFERENCES policies (id)  NOT NULL,
    key       VARCHAR2(128)                     NOT NULL,
    value     VARCHAR2(256)                     NOT NULL,
    UNIQUE (parent_id, key)
);
CREATE INDEX IF NOT EXISTS fk_policy_tags__parent_id ON policy_tags (parent_id ASC);
-- Role
CREATE TABLE IF NOT EXISTS roles
(
    id                          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id                  INTEGER REFERENCES accounts (id)  NOT NULL,
    role_name                   VARCHAR2(64)                      NOT NULL,
    unique_role_name            VARCHAR2(64)                      NOT NULL,
    description                 VARCHAR2(1000),
    max_session_duration        INTEGER                           NOT NULL,
    assume_role_policy_document VARCHAR2(131072)                  NOT NULL,
    arn                         VARCHAR2(2048)                    NOT NULL,
    path                        VARCHAR2(512)                     NOT NULL,
    role_id                     VARCHAR2(21)                      NOT NULL,
    policy_id                   INTEGER REFERENCES policies (id),
    create_date                 INTEGER                           NOT NULL,
    last_used_date              INTEGER,
    last_used_region_id         INTEGER REFERENCES regions (id),
    UNIQUE (arn),
    UNIQUE (role_id),
    UNIQUE (account_id, unique_role_name)
);
CREATE INDEX IF NOT EXISTS fk_roles__account_id ON roles (account_id ASC);
CREATE INDEX IF NOT EXISTS idx_roles__arn ON roles (arn ASC);
CREATE TABLE IF NOT EXISTS role_inline_policies
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    parent_id          INTEGER REFERENCES roles (id)     NOT NULL,
    policy_name        VARCHAR2(128)                     NOT NULL,
    unique_policy_name VARCHAR2(128)                     NOT NULL,
    policy_document    VARCHAR2(131072)                  NOT NULL,
    UNIQUE (parent_id, unique_policy_name)
);
CREATE INDEX IF NOT EXISTS fk_role_inline_policies__parent_id ON role_inline_policies (parent_id ASC);
CREATE TABLE IF NOT EXISTS role_tags
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    parent_id INTEGER REFERENCES roles (id)     NOT NULL,
    key       VARCHAR2(128)                     NOT NULL,
    value     VARCHAR2(256)                     NOT NULL,
    UNIQUE (parent_id, key)
);
CREATE INDEX IF NOT EXISTS fk_role_tags__parent_id ON role_tags (parent_id ASC);
CREATE TABLE IF NOT EXISTS group_users
(
    group_id INTEGER REFERENCES groups (id) NOT NULL,
    user_id  INTEGER REFERENCES users (id)  NOT NULL,
    UNIQUE (group_id, user_id) ON CONFLICT IGNORE
);
CREATE INDEX IF NOT EXISTS fk_group_users__group_id ON group_users (group_id ASC);
CREATE INDEX IF NOT EXISTS fk_group_users__user_id ON group_users (user_id ASC);
CREATE TABLE IF NOT EXISTS policy_groups
(
    policy_id INTEGER REFERENCES policies (id) NOT NULL,
    group_id  INTEGER REFERENCES groups (id)   NOT NULL,
    UNIQUE (policy_id, group_id) ON CONFLICT IGNORE
);
CREATE INDEX IF NOT EXISTS fk_policy_groups__group_id ON policy_groups (group_id ASC);
CREATE INDEX IF NOT EXISTS fk_policy_groups__policy_id ON policy_groups (policy_id ASC);
CREATE TABLE IF NOT EXISTS policy_roles
(
    policy_id INTEGER REFERENCES policies (id) NOT NULL,
    role_id   INTEGER REFERENCES roles (id)    NOT NULL,
    UNIQUE (policy_id, role_id) ON CONFLICT IGNORE
);
CREATE INDEX IF NOT EXISTS fk_policy_roles__policy_id ON policy_roles (policy_id ASC);
CREATE INDEX IF NOT EXISTS fk_policy_roles__role_id ON policy_roles (role_id ASC);
CREATE TABLE IF NOT EXISTS policy_users
(
    policy_id INTEGER REFERENCES policies (id) NOT NULL,
    user_id   INTEGER REFERENCES users (id)    NOT NULL,
    UNIQUE (policy_id, user_id) ON CONFLICT IGNORE
);
CREATE INDEX IF NOT EXISTS fk_policy_users__policy_id ON policy_users (policy_id ASC);
CREATE INDEX IF NOT EXISTS fk_policy_users__user_id ON policy_users (user_id ASC);
-- Instance profile
CREATE TABLE IF NOT EXISTS instance_profiles
(
    id                           INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id                   INTEGER REFERENCES accounts (id)  NOT NULL,
    instance_profile_name        VARCHAR2(128)                     NOT NULL,
    unique_instance_profile_name VARCHAR2(128)                     NOT NULL,
    instance_profile_id          VARCHAR(21)                       NOT NULL,
    arn                          VARCHAR2(2048)                    NOT NULL,
    path                         VARCHAR2(512)                     NOT NULL,
    create_date                  INTEGER                           NOT NULL,
    last_used_date               INTEGER,
    last_used_region_id          INTEGER REFERENCES regions (id),
    UNIQUE (account_id, unique_instance_profile_name),
    UNIQUE (arn)
);
CREATE INDEX IF NOT EXISTS fk_instance_profiles__account_id ON instance_profiles (account_id ASC);
CREATE INDEX IF NOT EXISTS idx_instance_profiles__arn ON instance_profiles (arn ASC);
CREATE TABLE IF NOT EXISTS instance_profile_roles
(
    instance_profile_id INTEGER REFERENCES instance_profiles (id) NOT NULL,
    role_id             INTEGER REFERENCES roles (id)             NOT NULL,
    UNIQUE (instance_profile_id, role_id) ON CONFLICT IGNORE
);
CREATE INDEX IF NOT EXISTS fk_instance_profile_roles__instance_profile_id ON instance_profile_roles (instance_profile_id ASC);
CREATE INDEX IF NOT EXISTS fk_instance_profile_roles__role_id ON instance_profile_roles (role_id ASC);
CREATE TABLE IF NOT EXISTS instance_profile_tags
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT         NOT NULL,
    parent_id INTEGER REFERENCES instance_profiles (id) NOT NULL,
    key       VARCHAR2(128)                             NOT NULL,
    value     VARCHAR2(256)                             NOT NULL,
    UNIQUE (parent_id, key)
);
CREATE INDEX IF NOT EXISTS fk_instance_profile_tags__parent_id ON instance_profile_tags (parent_id ASC);
CREATE TABLE IF NOT EXISTS login_profiles
(
    id                      INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id                 INTEGER REFERENCES users (id)     NOT NULL,
    create_date             INTEGER                           NOT NULL,
    password_hash           VARCHAR2(200)                     NOT NULL,
    password_reset_required BOOLEAN                           NOT NULL,
    UNIQUE (user_id)
);
CREATE INDEX IF NOT EXISTS fk_login_profiles__user_id ON login_profiles (user_id ASC);
-- OpenID Connect Providers
CREATE TABLE IF NOT EXISTS open_id_connect_providers
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id  INTEGER REFERENCES accounts (id)  NOT NULL,
    arn         VARCHAR2(2048)                    NOT NULL,
    url         VARCHAR2(255)                     NOT NULL,
    create_date INTEGER                           NOT NULL,
    UNIQUE (account_id, url),
    UNIQUE (arn)
);
CREATE INDEX IF NOT EXISTS fk_open_id_connect_providers__account_id ON open_id_connect_providers (account_id ASC);
CREATE TABLE IF NOT EXISTS open_id_connect_provider_client_ids
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT                 NOT NULL,
    provider_id INTEGER REFERENCES open_id_connect_providers (id) NOT NULL,
    client_id   VARCHAR2(255)                                     NOT NULL,
    UNIQUE (provider_id, client_id)
);
CREATE INDEX IF NOT EXISTS fk_open_id_connect_provider_client_ids__provider_id ON open_id_connect_provider_client_ids (provider_id ASC);
CREATE TABLE IF NOT EXISTS open_id_connect_provider_thumbprints
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT                 NOT NULL,
    provider_id INTEGER REFERENCES open_id_connect_providers (id) NOT NULL,
    thumbprint  VARCHAR(40)                                       NOT NULL,
    UNIQUE (provider_id, thumbprint)
);
CREATE INDEX IF NOT EXISTS fk_open_id_connect_provider_thumbprints__provider_id ON open_id_connect_provider_thumbprints (provider_id ASC);
CREATE TABLE IF NOT EXISTS open_id_connect_provider_tags
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT                 NOT NULL,
    parent_id INTEGER REFERENCES open_id_connect_providers (id) NOT NULL,
    key       VARCHAR2(128)                                     NOT NULL,
    value     VARCHAR2(256)                                     NOT NULL,
    UNIQUE (parent_id, key)
);
CREATE INDEX IF NOT EXISTS fk_open_id_connect_provider_tags__parent_id ON open_id_connect_provider_tags (parent_id ASC);
-- SAML provider
CREATE TABLE IF NOT EXISTS saml_providers
(
    id                INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id        INTEGER REFERENCES accounts (id)  NOT NULL,
    name              VARCHAR2(128)                     NOT NULL,
    unique_name       VARCHAR2(128)                     NOT NULL,
    arn               VARCHAR2(2048)                    NOT NULL,
    create_date       INTEGER                           NOT NULL,
    valid_until       INTEGER,
    metadata_document VARCHAR2(10000000)                NOT NULL,
    UNIQUE (arn),
    UNIQUE (account_id, unique_name)
);
CREATE INDEX IF NOT EXISTS fk_saml_providers__account_id ON saml_providers (account_id ASC);
CREATE TABLE IF NOT EXISTS saml_provider_tags
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT      NOT NULL,
    parent_id INTEGER REFERENCES saml_providers (id) NOT NULL,
    key       VARCHAR2(128)                          NOT NULL,
    value     VARCHAR2(256)                          NOT NULL,
    UNIQUE (parent_id, key)
);
CREATE INDEX IF NOT EXISTS fk_saml_provider_tags__parent_id ON saml_provider_tags (parent_id ASC);
-- Virtual MFA
CREATE TABLE IF NOT EXISTS mfa_devices
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id    INTEGER REFERENCES accounts (id)  NOT NULL,
    serial_number VARCHAR2(256)                     NOT NULL,
    path          VARCHAR2(512)                     NOT NULL,
    name          VARCHAR2(128)                     NOT NULL,
    unique_name   VARCHAR2(128)                     NOT NULL,
    seed          BLOB                              NOT NULL,
    code1         VARCHAR2(6),
    code2         VARCHAR2(6),
    create_date   INTEGER                           NOT NULL,
    enable_date   INTEGER,
    user_id       INTEGER REFERENCES users (id), -- MFA device could be assigned to only one user
    UNIQUE (serial_number),
    UNIQUE (account_id, unique_name)
);
CREATE INDEX IF NOT EXISTS fk_mfa_devices__account_id ON mfa_devices (account_id ASC);
CREATE TABLE IF NOT EXISTS mfa_device_tags
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT   NOT NULL,
    parent_id INTEGER REFERENCES mfa_devices (id) NOT NULL,
    key       VARCHAR2(128)                       NOT NULL,
    value     VARCHAR2(256)                       NOT NULL,
    UNIQUE (parent_id, key)
);
CREATE INDEX IF NOT EXISTS fk_mfa_device_tags__parent_id ON mfa_device_tags (parent_id ASC);