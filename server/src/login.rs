use hyper::body::Buf;
use hyper::client::HttpConnector;
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use sha_crypt::{sha512_check, sha512_simple, Sha512Params};
use sqlx::mysql::MySqlPool;
use sqlx::Row;
use sqlx::{MySql, Pool};
use std::collections::HashMap;
use tokio_postgres::{Error, NoTls};
type JsonMap = HashMap<String, serde_json::Value>;
use std::env;
type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;

#[derive(Serialize, Deserialize,Debug)]

pub enum LoginType {
    Username,
    PhoneStep1,
    PhoneStep2,
}
#[derive(Serialize, Deserialize,Debug)]
pub struct LoginRequest {
    pub login_type: LoginType,
    pub login_input: String,
    pub login_credentials: String,
}

pub fn validate_username(username: &str) -> bool {
    if username.len() > 7 {
        return true;
    }
    false
}

pub async fn login_proccess<'a>(
    req: Request<Body>,
    mut pool: sqlx::pool::PoolConnection<sqlx::mysql::MySql>,
) -> (&'a str, u16) {
    // Aggregate the body...
    let whole_body = hyper::body::aggregate(req).await.unwrap();
    // Decode as JSON ...
    let mut data: Option<LoginRequest> = match serde_json::from_reader(whole_body.reader()) {
        Ok(data) => Some(data),
        _ => None,
    };
    println!("data req is : {:?}", data);
    match data {
        Some(data) => {
            match data.login_type {
                LoginType::Username => {
                    match sqlx::query!(
                        "SELECT password FROM user WHERE username=?;",
                        data.login_input
                    )
                    .fetch_all(&mut pool)
                    .await
                    {
                        Ok(database) => {
                            // Check username validation...
                            if validate_username(&data.login_input) {
                                if database.len() < 1 {
                                    ("Please Sign up", 200)
                                } else {
                                    // Verifying a stored password
                                    match sha512_check(
                                        &data.login_credentials,
                                        &database[0].password[..],
                                    ) {
                                        Ok(_) => ("Now you are logged in", 200),
                                        _ => ("Wrong password", 200),
                                    }
                                    // ("Wrong password", 200)
                                }
                            } else {
                                ("Invalid username", 200)
                            }
                        }
                        _ => ("internal Error", 500),
                    }
                }
                _ => ("internal Error", 500),
            }
        }
        None => ("Invalid Input", 500),
    }
}
#[cfg(test)]
#[test]
fn test_hash() {
    // rounds = 10_000
    let params = Sha512Params::new(10_000).expect("RandomError!");

    // Hash the password for storage
    let hashed_password = sha512_simple("12345678", &params).expect("Should not fail");
    assert_eq!(sha512_check("12345678", &hashed_password[..]).is_ok(), true);
}
#[test]
fn test_hash_wrong() {
    // rounds = 10_000
    let params = Sha512Params::new(10_000).expect("RandomError!");

    // Hash the password for storage
    let hashed_password = sha512_simple("sdfsdf", &params).expect("Should not fail");
    assert_eq!(
        sha512_check("12345678", &hashed_password[..]).is_ok(),
        false
    );
}
#[tokio::test]
async fn test_login_signup_request() {
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&database_url[..]).await.unwrap();
    let request_json = LoginRequest {
        login_type: LoginType::Username,
        login_input: String::from("farbodpm"),
        login_credentials: String::from("12345678"),
    };
    let request_budy = serde_json::to_string(&request_json).unwrap();
    let req = Request::new(Body::from(request_budy));
    let (message, status) = login_proccess(req, pool.acquire().await.unwrap()).await;
    assert_eq!(status, 200);
    assert_eq!(message, "Please Sign up");
}
#[tokio::test]
async fn test_login_correct_request() {
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&database_url[..]).await.unwrap();
    // rounds = 10_000
    let params = Sha512Params::new(10_000).expect("RandomError!");

    // Hash the password for storage
    let hashed_password = sha512_simple("12345678", &params).expect("Should not fail");
    println!("{:?}", hashed_password);
    sqlx::query("INSERT INTO user(username, password, email ) VALUES(\"farbodpm\", ? , \"farbod@gmail.com\");").bind(hashed_password).execute(&pool).await.unwrap();
    let request_json = LoginRequest {
        login_type: LoginType::Username,
        login_input: String::from("farbodpm"),
        login_credentials: String::from("12345678"),
    };
    let request_budy = serde_json::to_string(&request_json).unwrap();
    let req = Request::new(Body::from(request_budy));
    let (message, status) = login_proccess(req, pool.acquire().await.unwrap()).await;
    sqlx::query("DELETE from user where username='farbodpm' ;")
        .execute(&pool)
        .await
        .unwrap();

    assert_eq!(status, 200);
    assert_eq!(message, "Now you are logged in");

    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&database_url[..]).await.unwrap();
    // rounds = 10_000
    let params = Sha512Params::new(10_000).expect("RandomError!");

    // Hash the password for storage
    let hashed_password = sha512_simple("87654321", &params).expect("Should not fail");
    println!("{:?}", hashed_password);
    sqlx::query("INSERT INTO user(username, password, email ) VALUES(\"farbodpm\", ? , \"farbod@gmail.com\");").bind(hashed_password).execute(&pool).await.unwrap();
    let request_json = LoginRequest {
        login_type: LoginType::Username,
        login_input: String::from("farbodpm"),
        login_credentials: String::from("12345678"),
    };
    let request_budy = serde_json::to_string(&request_json).unwrap();
    let req = Request::new(Body::from(request_budy));
    let (message, status) = login_proccess(req, pool.acquire().await.unwrap()).await;
    sqlx::query("DELETE from user where username='farbodpm' ;")
        .execute(&pool)
        .await
        .unwrap();

    assert_eq!(status, 200);
    assert_eq!(message, "Wrong password");
}
#[tokio::test]
async fn test_login_wrong_input() {
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&database_url[..]).await.unwrap();
    let request_json = LoginRequest {
        login_type: LoginType::Username,
        login_input: String::from("farbod"),
        login_credentials: String::from(""),
    };
    let request_budy = serde_json::to_string(&request_json).unwrap();
    let req = Request::new(Body::from(request_budy));
    let (message, status) = login_proccess(req, pool.acquire().await.unwrap()).await;
    assert_eq!(status, 200);
    assert_eq!(message, "Invalid username");
}
