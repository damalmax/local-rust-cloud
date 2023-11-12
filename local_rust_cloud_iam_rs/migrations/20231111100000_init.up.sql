-- Account
CREATE TABLE IF NOT EXISTS accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    account_id INTEGER NOT NULL,
    alias VARCHAR2(63) NOT NULL
);
INSERT INTO accounts(account_id, alias)
VALUES (123456789012, 'Local-Rust-Cloud-user');
-- Region
CREATE TABLE IF NOT EXISTS regions (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR2(10) NOT NULL
);
INSERT INTO regions(id, name)
VALUES (1, 'us-east-1'),
    (2, 'eu-local-1');
-- User
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    username VARCHAR2(20) NOT NULL,
    path VARCHAR2(512) NOT NULL,
    user_id VARCHAR2(20) NOT NULL,
    account_id INTEGER REFERENCES accounts (id),
    region_id INTEGER REFERENCES regions (id),
    UNIQUE(user_id)
);
INSERT INTO users(username, path, user_id, account_id, region_id)
VALUES ('Admin', "/", "AKIAIOSFODNN101ADMIN", 1, 1),
    ('Admin', "/", "AKIAIOSFODNN201ADMIN", 1, 2);