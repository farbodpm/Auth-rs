use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Client, Method, Request, Response, Server, StatusCode};
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use sqlx::{MySql, Pool};
use std::collections::HashMap;
use std::env;
use tokio::runtime::Runtime;
use tokio_postgres::{Error, NoTls};
type JsonMap = HashMap<String, serde_json::Value>;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
mod login;
mod signup;
static INDEX: &[u8] = b"<a href=\"test.html\">test.html</a>";
static NOTFOUND: &[u8] = b"Some Thing is Wrong";
static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
#[derive(Serialize, Deserialize)]
struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub fullname: String,
    pub family: String,
    pub email: String,
    pub image: String,
    pub created_at: String,
}
#[derive(Serialize, Deserialize)]
struct Token {
    pub id: u64,
    pub token: String,
    pub created_at: String,
}

async fn login(
    req: Request<Body>,
    mut pool: sqlx::pool::PoolConnection<sqlx::mysql::MySql>,
) -> Result<Response<Body>> {
    let (message, status) = crate::login::login_proccess(req, pool).await;

    Ok(Response::builder()
        // .header(header::CONTENT_TYPE, "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Credentials", "true")
        .header(header::CONTENT_TYPE, "application/json")
        .status(status)
        .body(Body::from(message))
        .unwrap())
}
async fn logout(req: Request<Body>) -> Result<Response<Body>> {
    let (message, status) = ("not impl", 200);

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .status(status)
        .body(Body::from(message))
        .unwrap())
}
async fn get_user(req: Request<Body>) -> Result<Response<Body>> {
    let (message, status) = ("not impl", 200);

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .status(status)
        .body(Body::from(message))
        .unwrap())
}

async fn reset(
    req: Request<Body>,
    mut pool: sqlx::pool::PoolConnection<sqlx::mysql::MySql>,
) -> Result<Response<Body>> {
    let (message, status) = ("not impl", 200);

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .status(status)
        .body(Body::from(message))
        .unwrap())
}
async fn signup(
    req: Request<Body>,
    mut pool: sqlx::pool::PoolConnection<sqlx::mysql::MySql>,
) -> Result<Response<Body>> {
    let (message, status) = crate::signup::signup_proccess(req, pool).await;

    Ok(Response::builder()
        // .header(header::CONTENT_TYPE, "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Credentials", "true")
        .header(header::CONTENT_TYPE, "application/json")
        .status(status)
        .body(Body::from(message))
        .unwrap())
}
use std::sync::{Arc, Mutex};

async fn response_examples(
    req: Request<Body>,
    client: Client<HttpConnector>,
    pool: PoolConnection<MySql>,
    time_req: Arc<Mutex<u128>>,
) -> Result<Response<Body>> {
    println!(" urll is {} ", req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/api/reset") => match pool.acquire().await {
            Ok(db_con) => reset(req, db_con).await,
            _ => Ok(Response::builder()
                .status(StatusCode::from_u16(500).unwrap())
                .body(NOTFOUND.into())
                .unwrap()),
        },
        (&Method::POST, "/api/login") => match pool.acquire().await {
            Ok(db_con) => login(req, db_con).await,
            _ => Ok(Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "*")
                .status(StatusCode::from_u16(500).unwrap())
                .body(NOTFOUND.into())
                .unwrap()),
        },
        (&Method::POST, "/api/signup") => match pool.acquire().await {
            Ok(db_con) => signup(req, db_con).await,
            _ => Ok(Response::builder()
                .status(StatusCode::from_u16(500).unwrap())
                .body(NOTFOUND.into())
                .unwrap()),
        },
        (&Method::POST, "/api/logout") => logout(req).await,
        (&Method::POST, "/api/get/user") => get_user(req).await,
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "*")
                .header("Access-Control-Allow-Headers", "*")
                .status(200)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}

extern crate dotenv;

use dotenv::dotenv;
// Read migrations from a local folder: ./migrations
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let time = std::time::SystemTime::now();
    let time = time.duration_since(std::time::UNIX_EPOCH).unwrap();
    let time_for_send = Arc::new(Mutex::new(time.as_millis()));

    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&database_url[..]).await?;
    // run migrations here
    // let state_migrate = sqlx::migrate!("./src/migrations").run(&pool).await;
    // println!("state migrate is : {:?} ", state_migrate);

    // match database_type {
    //     DatabaseType::Mysql => {}
    //     DatabaseType::Postgresql => {
    //         let (client, connection) =
    //             tokio_postgres::connect("host=localhost user=postgres password=farbod", NoTls)
    //                 .await?;
    //         // The connection object performs the actual communication with the database,
    //         // so spawn it off to run on its own.
    //         tokio::spawn(async move {
    //             if let Err(e) = connection.await {
    //                 eprintln!("connection error: {}", e);
    //             }
    //         });
    //     }
    // }
    // Create the runtime
    // let mut rt = Runtime::new().unwrap();

    // rt.spawn(async move {
    //     let mut connection = pool.acquire().await.unwrap();
    //     let rows = match sqlx::query!(
    //         "
    //     SELECT * FROM requests WHERE status = 'failed'"
    //     )
    //     .fetch_all(&mut connection)
    //     .await
    //     {
    //         Ok(t) => Some(t),
    //         _ => None,
    //     };
    //     println!("row is {:?}", rows);
    // });
    // Share a `Client` with all `Service`s
    let client = Client::new();

    let new_service = make_service_fn(move |_| {
        // Move a clone of `client` into the `service_fn`.
        let client = client.clone();
        let pooll = pool.clone().acquire().await.unwrap();
        let req_to_divar = time_for_send.clone();
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                // Clone again to ensure that client outlives this closure.
                response_examples(
                    req,
                    client.to_owned(),
                    pooll.to_owned(),
                    req_to_divar.to_owned(),
                )
            }))
        }
    });
    let addr = "127.0.0.1:8000".parse().unwrap();
    let server = Server::bind(&addr).serve(new_service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
