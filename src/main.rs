use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Client, Method, Request, Response, Server, StatusCode};
use sqlx::mysql::MySqlPool;
use sqlx::{ MySql, Pool};
use std::env;
use hyper::body::Buf;
use tokio::runtime::Runtime;
use tokio_postgres::{Error, NoTls};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type JsonMap = HashMap<String, serde_json::Value>;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
static INDEX: &[u8] = b"<a href=\"test.html\">test.html</a>";
static NOTFOUND: &[u8] = b"Some Thing is Wrong";
static INTERNAL_SERVER_ERROR: &[u8] = b"Internal Server Error";
#[derive(Serialize, Deserialize)]
struct User{
    pub id             : u64,
    pub username       : String,
    pub password       : String,
    pub fullname       : String,
    pub family         : String,
    pub email          : String,
    pub image          : String,
    pub created_at  : String,
}
#[derive(Serialize, Deserialize)]
struct Token{
    pub id             : u64,
    pub token       : String,
    pub created_at  : String,
}

async fn login(        req: Request<Body>,
) -> Result<Response<Body>> {
    
    let res = match serde_json::to_string(&user) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap(),
    };
    Ok(res)
}
async fn logout(        req: Request<Body>,
) -> Result<Response<Body>> {
            
    let res = match serde_json::to_string(&user) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap(),
    };
    Ok(res)
}
async fn get_user(        req: Request<Body>,
) -> Result<Response<Body>> {
            
    let res = match serde_json::to_string(&user) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap(),
    };
    Ok(res)
}

async fn reset(        req: Request<Body>,
) -> Result<Response<Body>> {
            
    let res = match serde_json::to_string(&user) {
        Ok(json) => Response::builder()
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(json))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(INTERNAL_SERVER_ERROR.into())
            .unwrap(),
    };
    Ok(res)
}
use std::sync::{Arc, Mutex};

async fn response_examples(
    req: Request<Body>,
    client: Client<HttpConnector>,
    pool: Pool<MySql>,
    time_req: Arc<Mutex<u128>>,
) -> Result<Response<Body>> {
    println!(" urll is {}", req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/api/reset") => reset(req).await,
        (&Method::POST, "/api/login") => login(req).await,
        (&Method::POST, "/api/logout") => logout(req).await,
        (&Method::POST, "/api/get/user") => get_user(req).await,
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}

extern crate dotenv;

use dotenv::dotenv;
use sqlx::pool::PoolConnection;
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
        let pooll = pool.clone();
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
    let addr = "0.0.0.0:80".parse().unwrap();
    let server = Server::bind(&addr).serve(new_service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}

// #[cfg(test)]
// #[test]
// fn test_divar_request() {
//     let payload = "{
//     \"json_schema\": {
//         \"cities\": [
//             \"4\"
//         ]
//     },
//     \"last-post-date\": 0
//     }";
//     let url = "https://api.divar.ir/v8/search/1/CCTV";

//     //request(payload, url);
// }
