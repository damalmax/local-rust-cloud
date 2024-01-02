-- Account
CREATE TABLE IF NOT EXISTS accounts
(
    id    INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    alias VARCHAR2(63)                      NOT NULL
);
INSERT INTO accounts(alias)
VALUES ('Local-Rust-Cloud-user');
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
-- User
CREATE TABLE IF NOT EXISTS users
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    username   VARCHAR2(20)                      NOT NULL,
    path       VARCHAR2(512)                     NOT NULL,
    user_id    VARCHAR2(20)                      NOT NULL,
    account_id INTEGER REFERENCES accounts (id),
    region_id  INTEGER REFERENCES regions (id),
    UNIQUE (user_id)
);
INSERT INTO users(username, path, user_id, account_id, region_id)
VALUES ('Admin', '/', 'LOCALCLOUDRS101ADMIN', 1, 2),
       ('Admin', '/', 'LOCALCLOUDRS201ADMIN', 1, 1);
-- Policy
CREATE TABLE IF NOT EXISTS policies
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id    INTEGER REFERENCES accounts (id)  NOT NULL,
    policy_name   VARCHAR2(128)                     NOT NULL,
    policy_id     VARCHAR2(21),
    arn           VARCHAR2(100),
    path          VARCHAR2(100),
    is_attachable BOOLEAN,
    description   VARCHAR2(200),
    create_date   INTEGER                           NOT NULL,
    update_date   INTEGER                           NOT NULL,
    UNIQUE (arn)
);

CREATE TABLE IF NOT EXISTS policy_versions
(
    id                INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id        INTEGER REFERENCES accounts (id)  NOT NULL,
    policy_id         INTEGER REFERENCES policies (id)  NOT NULL,
    policy_version_id VARCHAR2(21)                      NOT NULL,
    policy_document   VARCHAR2(6144)                    NOT NULL,
    version           INTEGER                           NOT NULL,
    create_date       INTEGER                           NOT NULL,
    is_default        BOOLEAN                           NOT NULL
);

CREATE TABLE IF NOT EXISTS policy_tags
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    policy_id INTEGER REFERENCES policies (id),
    key       VARCHAR2(100)                     NOT NULL,
    value     VARCHAR2(100)                     NOT NULL,
    UNIQUE (policy_id, key)
);

CREATE TABLE IF NOT EXISTS unique_identifiers
(
    id            INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    unique_id     VARCHAR2(21)                      NOT NULL,
    resource_type INTEGER                           NOT NULL,
    UNIQUE (unique_id)
);