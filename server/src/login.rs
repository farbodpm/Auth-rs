use hyper::body::Buf;
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};
use serde::{Deserialize, Serialize};
use sha_crypt::{sha512_check, sha512_simple, Sha512Params};
use sqlx::mysql::MySqlPool;
use sqlx::Row;
use std::collections::HashMap;
use jwt_simple::prelude::HS256Key;
use jwt_simple::prelude::Claims;
use jwt_simple::prelude::*;
use std::env;

pub const KEY : [u8; 32] = [98, 46, 158, 67, 91, 230, 115, 47, 193, 232, 204, 147, 231, 64, 33, 83, 57, 89, 76, 11, 125, 127, 187, 210, 59, 20, 4, 167, 245, 218, 222, 
51];

#[derive(Serialize, Deserialize, Debug)]

pub enum LoginType {
    Username,
    PhoneStep1,
    PhoneStep2,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub login_type: LoginType,
    pub login_input: String,
    pub login_credentials: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
    pub name : String
}
#[derive(Serialize, Deserialize)]
pub struct MyAdditionalData {
   pub user_id: u64,
//    companies: Vec<u64>,
}
pub fn validate_username(username: &str) -> bool {
    let mut res = true;
    if username.len() == 9 {
        for c in username.chars() {
            if !c.is_numeric() {
                res = false;
                break;
            }
        }
    } else {
        res = false;
    }
    res
}

pub fn validate_password(password: &str) -> bool {
    if password.len() > 7 {
        return true;
    }
    false
}

pub async fn login_proccess(
    req: Request<Body>,
    mut pool: sqlx::pool::PoolConnection<sqlx::mysql::MySql>,
) -> (String, u16) {
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
                        "SELECT id,password , username FROM user WHERE username=?;",
                        data.login_input
                    )
                    .fetch_all(&mut pool)
                    .await
                    {
                        Ok(database) => {
                            // Check username validation...
                            if validate_username(&data.login_input) {
                                if database.len() < 1 {
                                    let mut response = LoginResponse{
                                        message: String::from("Please Sign up"),
                                        token: String::from(""),
                                        name : String::from(""),
                                    };
                                    
                                    (serde_json::to_string(&response).unwrap(), 200)
                                } else {
                                    // Verifying a stored password
                                    match sha512_check(
                                        &data.login_credentials,
                                        &database[0].password[..],
                                    ) {
                                        Ok(_) => {
                                            let my_additional_data = MyAdditionalData{user_id : database[0].id};
                                            let claims = Claims::with_custom_claims(my_additional_data, Duration::from_mins(180));
                                            let key = HS256Key::from_bytes(&KEY);
                                            let token = key.authenticate(claims).unwrap();

                                            let mut response = LoginResponse{
                                                message: String::from("Now you are logged in"),
                                                token: String::from(token),
                                                name : String::from(&database[0].username)
                                            };
                                            let resp_str = serde_json::to_string(&response).unwrap();
                                            (resp_str, 200)
                                            //todo add hash code in token
                                            },
                                        _ => (String::from("Wrong password"), 200),
                                    }
                                    // ("Wrong password", 200)
                                }
                            } else {
                                (String::from("Invalid username"), 200)
                            }
                        }
                        _ => (String::from("internal Error"), 500),
                    }
                }
                _ => (String::from("internal Error"), 500),
            }
        }
        None => (String::from("Invalid Input"), 500),
    }
}
#[cfg(test)]
#[test]
fn test_jwt(){
    let my_additional_data = MyAdditionalData{user_id : 1};
    let claims = Claims::with_custom_claims(my_additional_data, Duration::from_mins(180));
    let key = HS256Key::from_bytes(&KEY);
    let token = key.authenticate(claims).unwrap();
    let claims = key.verify_token::<MyAdditionalData>(&token, None);
    match claims {
        Ok(my_data) =>{
        } ,
        Err(e)=> panic!("token is {:?} , Error {}",token,e)
    };

}
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
        login_input: String::from("132147966"),
        login_credentials: String::from("12345678"),
    };
    let request_budy = serde_json::to_string(&request_json).unwrap();
    let req = Request::new(Body::from(request_budy));
    let (message, status) = login_proccess(req, pool.acquire().await.unwrap()).await;
    assert_eq!(status, 200);
    assert_eq!(message, String::from("Please Sign up"));
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
    sqlx::query("INSERT INTO user(username, password, email ) VALUES(\"132147966\", ? , \"farbod@gmail.com\");").bind(hashed_password).execute(&pool).await.unwrap();
    let request_json = LoginRequest {
        login_type: LoginType::Username,
        login_input: String::from("132147966"),
        login_credentials: String::from("12345678"),
    };
    let request_budy = serde_json::to_string(&request_json).unwrap();
    let req = Request::new(Body::from(request_budy));
    let (message, status) = login_proccess(req, pool.acquire().await.unwrap()).await;
    sqlx::query("DELETE from user where username='132147966' ;")
        .execute(&pool)
        .await
        .unwrap();

    assert_eq!(status, 200);
    let msg : LoginResponse = serde_json::from_str(&message).unwrap();
    assert_eq!(msg.message, "Now you are logged in");

    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&database_url[..]).await.unwrap();
    // rounds = 10_000
    let params = Sha512Params::new(10_000).expect("RandomError!");

    // Hash the password for storage
    let hashed_password = sha512_simple("87654321", &params).expect("Should not fail");
    println!("{:?}", hashed_password);
    sqlx::query("INSERT INTO user(username, password, email ) VALUES(\"132147966\", ? , \"farbod@gmail.com\");").bind(hashed_password).execute(&pool).await.unwrap();
    let request_json = LoginRequest {
        login_type: LoginType::Username,
        login_input: String::from("132147966"),
        login_credentials: String::from("12345678"),
    };
    let request_budy = serde_json::to_string(&request_json).unwrap();
    let req = Request::new(Body::from(request_budy));
    let (message, status) = login_proccess(req, pool.acquire().await.unwrap()).await;
    sqlx::query("DELETE from user where username='132147966' ;")
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
