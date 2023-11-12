CREATE TABLE IF NOT EXISTS accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id INTEGER NOT NULL,
    alias VARCHAR2(63) NOT NULL
);
INSERT INTO accounts(account_id, alias)
VALUES (123456789012, 'Local-Rust-Cloud-user');
CREATE TABLE IF NOT EXISTS sts_roles (arn VARCHAR2(200) PRIMARY KEY NOT NULL);
CREATE TABLE IF NOT EXISTS regions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR2(10) NOT NULL
);
INSERT INTO regions(name)
VALUES ('us-east-1'),
    ('eu-local-1');
CREATE TABLE IF NOT EXISTS credentials (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    access_key_id VARCHAR2(30) NOT NULL,
    secret_access_key VARCHAR2(300) NOT NULL,
    session_token VARCHAR2(400) NOT NULL,
    expiration INTEGER NOT NULL,
    account_id INTEGER REFERENCES accounts (id),
    region_id INTEGER REFERENCES regions (id)
);
