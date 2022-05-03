CREATE TABLE IF NOT EXISTS user
(
    id             BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    username       TEXT    NOT NULL UNIQUE,
    password       TEXT    NOT NULL,
    fullname       TEXT   ,
    family         TEXT   ,
    email          TEXT   ,
    image          TEXT   ,
    created_at  DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS user_token
(
    id             BIGINT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
    token          TEXT    NOT NULL UNIQUE,
    created_at     DATETIME DEFAULT CURRENT_TIMESTAMP
);
