

pub mod get{
    use hyper::body::Buf;
use sqlx::mysql::MySqlPool;
use std::env;
use serde::Serialize;
#[derive(Serialize)]
struct Result{
    money : u64
}
pub async fn get<'a>(
    user_id: u64,
    mut pool: sqlx::pool::PoolConnection<sqlx::mysql::MySql>,
) -> u64 {
    match sqlx::query!(
        "SELECT * FROM user_wallet WHERE user_id=?;",
        user_id
    )
    .fetch_one(&mut pool)
    .await
    {
        Ok(value) => value.value as u64,
        _ => 0
    }
}
pub async fn handler<'a>(
    user_id: u64,
    mut pool: sqlx::pool::PoolConnection<sqlx::mysql::MySql>,
) -> (String, u16)
{
    let money = get(user_id, pool).await;
    let res = Result{
        money: money,
    };
    (serde_json::to_string(&res).unwrap(),200)
}
#[cfg(test)]
#[tokio::test]
async fn test_get_wallet() {
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&database_url[..]).await.unwrap();
    sqlx::query("INSERT INTO user_wallet(user_id,value) VALUES(1 ,1000);").execute(&pool).await.unwrap();

    let data = get(1, pool.acquire().await.unwrap()).await;
    sqlx::query("DELETE FROM user_wallet WHERE user_id=1;").execute(&pool).await.unwrap();
    assert_eq!(data, 1000);
}
#[tokio::test]
async fn test_get_wallet_no_row() {
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&database_url[..]).await.unwrap();
    let data = get(1, pool.acquire().await.unwrap()).await;
    assert_eq!(data, 0);
}
}