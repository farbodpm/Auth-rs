use hyper::body::Buf;
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use sqlx::Row;
use std::env;


#[derive(Serialize, Deserialize, Debug)]
pub struct LogoutRequest {
    pub token: String,
}
pub async fn logout<'a>(req: Request<Body>
,    mut pool: sqlx::pool::PoolConnection<sqlx::mysql::MySql>,

) -> (&'a str, u16) {
    // Aggregate the body...
    let whole_body = hyper::body::aggregate(req).await.unwrap();
    // Decode as JSON ...
    let mut request: Option<LogoutRequest> = match serde_json::from_reader(whole_body.reader()) {
        Ok(data) => Some(data),
        _ => None,
    };
    match request {
        Some(data) => {
            match sqlx::query!(
                "DELETE  FROM user_token WHERE token=?;",
                data.token
            )
            .execute(&mut pool)
            .await
            {
                Ok(result) => {
                    if result.rows_affected() !=1 {
                        // TODO: log an invalid logout request
                    }
                    ("Logout",200)

                }
                _ => ("internal error",500)
            }
        }
        _ => ("internal error",500)
    }

}