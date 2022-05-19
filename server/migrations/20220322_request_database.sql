CREATE TABLE IF NOT EXISTS user
(
    id             BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    username       TEXT    NOT NULL , 
    password       TEXT    NOT NULL,
    fullname       TEXT   ,
    family         TEXT   ,
    email          TEXT   ,
    image          TEXT   ,
    status         ENUM("Blocked","Created","Hidden","Completed"),
    created_at  DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS user_attr
(
    id             BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    user_id       TEXT    NOT NULL , 
    type       TEXT    NOT NULL,
    value       TEXT    NOT NULL,
    created_at  DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS user_company
(
    id          BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    company_id BIGINT UNSIGNED,
    user_id BIGINT UNSIGNED,
    created_at  DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS company
(
    id          BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    name        TEXT NOT NULL , 
    address     TEXT NOT NULL,
    lat         TEXT   ,
    lng         TEXT   ,
    phone       TEXT   ,
    created_at  DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS user_token
(
    id             BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    token          TEXT    NOT NULL UNIQUE,
    created_at     DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS user_wallet
(
    id             BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    user_id        BIGINT NOT NULL,
    value           BIGINT NOT NULL
);
CREATE TABLE IF NOT EXISTS transaction
(
    id             BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    user_id        BIGINT NOT NULL,
    company_id     BIGINT NOT NULL,
    value          BIGINT NOT NULL,
    description    TEXT NOT NULL,
    method         ENUM("cash","card","online")
    status         ENUM("done","need_for_action","rejected")
);