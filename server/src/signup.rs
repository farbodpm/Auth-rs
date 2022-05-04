use hyper::body::Buf;
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};
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

#[derive(Deserialize, Serialize,Debug)]
pub struct SignupRequest{
    username: String,
    password: String,

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
        println!("data req is : {:?}", data);
        match data {
            Some(data) => {
                ("not implemented",200)
            }
            None => ("Invalid Input", 500),
        }
}
