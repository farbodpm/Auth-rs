use hyper::body::Buf;
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};
use serde::{Deserialize, Serialize};
use sha_crypt::{sha512_check, sha512_simple, Sha512Params};
use sqlx::mysql::MySqlPool;
use std::env;
use sqlx::{MySql, Pool};
use std::collections::HashMap;
use tokio_postgres::{Error, NoTls};
use crate::login::{validate_password,validate_username};

#[derive(Deserialize, Serialize, Debug)]
pub struct SignupRequest {
    username: String, // this should be phone number
    password: String,
    //todo token handle
}

pub async fn signup_proccess<'a>(
    req: Request<Body>,
    mut pool: sqlx::pool::PoolConnection<sqlx::mysql::MySql>,
) -> (&'a str, u16) {
    // Aggregate the body...
    let whole_body = hyper::body::aggregate(req).await.unwrap();
    // Decode as JSON ...
    let mut data: Option<SignupRequest> = match serde_json::from_reader(whole_body.reader()) {
        Ok(data) => Some(data),
        _ => None,
    };
    println!("signup data req is : {:?}", data);
    match data {
        Some(data) => {
            // Validation of username and password
            match (validate_username(&data.username),validate_password(&data.password)) {
                (true, true) => {
                    // rounds = 10_000
                let params = Sha512Params::new(10_000).expect("RandomError!");

                // Hash the password for storage
                let hashed_password =
                    sha512_simple(&data.password, &params).expect("Should not fail");

                    // Sign up
                    match sqlx::query(
                        "INSERT INTO user(username, password,status) VALUES( ? , ? , \"Created\") ",
                    )
                    .bind(data.username)
                    .bind(hashed_password)
                    .execute(&mut pool)
                    .await{
                        Err(_e) => {
                            println!("{:?}",_e);
                            ("Already exist", 200)},
                        _ => ("Success", 200)
                    }
                }
                (_, _) => ("Invalid username or password", 200),
            }
        }
        None => ("Invalid Input", 500),
    }
}
#[cfg(test)]
#[tokio::test]
async fn test_signup_proccess_signup_successful() {
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&database_url[..]).await.unwrap();
    let request_json = SignupRequest {
        username: String::from("132147966"),
        password: String::from("12345678"),
    };
    let request_budy = serde_json::to_string(&request_json).unwrap();
    let req = Request::new(Body::from(request_budy));
    let res = signup_proccess(req, pool.acquire().await.unwrap()).await;
    assert_eq!(res,("Success",200));
    let request_json = crate::login::LoginRequest {
        login_type: crate::login::LoginType::Username,
        login_input: String::from("132147966"),
        login_credentials: String::from("12345678"),
    };
    let request_budy = serde_json::to_string(&request_json).unwrap();
    let req = Request::new(Body::from(request_budy));
    let (message, status) = crate::login::login_proccess(req, pool.acquire().await.unwrap()).await;
    assert_eq!(status, 200);
    assert_eq!(message, "Now you are logged in");
    let count = sqlx::query("DELETE FROM user WHERE username=\"132147966\"").execute(&mut pool.acquire().await.unwrap()).await.unwrap().rows_affected();
    assert_eq!(count,1);
}
