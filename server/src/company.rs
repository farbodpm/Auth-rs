use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Company{
        id : u64,
        name : String, 
        address :  String, 
        lat : Option<String>, 
        lng :  Option<String>, 
        phone  :  Option<String>, 
        // created_at :  String,
    }
    impl Company {
        pub fn new(id: u64,
            name: String,
            address:  String,
            lat: Option<String>,lng: Option<String>,
            phone: Option<String>,
            ) -> Self {
                Self { id, name, address, lat, lng, phone}
            }
    }

pub mod create {
    use hyper::body::Buf;
    use hyper::body::Body;
    use sqlx::mysql::MySqlPool;
    use std::env;
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Debug)]
    struct Request{
        name : String, 
        address :  String, 
        lat : String, 
        lng :  String, 
        phone  :  String, 
    }
    impl Request {
        fn new(name : String, address : String, lat : String, lng: String, phone: String) -> Self 
        {
            Self{
                name,
                address,
                lat,
                lng,
                phone,
            }
        }
    }
    #[derive(Deserialize, Serialize)]
    struct Result{
        message : String,
    }
    pub async fn handler<'a>(
        body: Body,
        mut pool: sqlx::pool::PoolConnection<sqlx::mysql::MySql>,
    ) -> (String, u16)
    {
        // Aggregate the body...
    let whole_body = hyper::body::aggregate(body).await.unwrap();
    // Decode as JSON ...
    let mut data: Option<Request> = match serde_json::from_reader(whole_body.reader()) {
        Ok(data) => Some(data),
        _ => None,
    };
    println!("create company data req is : {:?}", data);
    match data {
        Some(data) => {
                // Create company
                match sqlx::query(
                    "INSERT INTO company(name, address,lat,lng, phone) VALUES( ? , ? , ? , ? , ?) ",
                )
                .bind(data.name)
                .bind(data.address)
                .bind(data.lat)
                .bind(data.lng)
                .bind(data.phone)
                .execute(&mut pool)
                .await{
                    Err(_e) => {
                        println!("{:?}",_e);
                        (String::from("Already exist"), 200)},
                    _ => (String::from("Success"), 200)
                }
            }
        None => (String::from("Invalid Input"), 500),
    }
}
    #[cfg(test)]
    #[tokio::test]
    async fn test_create_company() {
        let database_url = env::var("DATABASE_URL").unwrap();
        let pool = MySqlPool::connect(&database_url[..]).await.unwrap();
        let request = Request::new(
            String::from("test"),
            String::from("Isfahan"),
            String::from(""),
            String::from(""),
            String::from("09132147966")
        );
        let req_body = Body::from(serde_json::to_string(&request).unwrap());
        let (message, status) = handler(req_body,pool.acquire().await.unwrap()).await;
        sqlx::query("DELETE from company where name='test' ;")
        .execute(&pool)
        .await
        .unwrap();
        assert_eq!(status, 200);
        assert_eq!(message,"Success")
    }
}
pub mod list {
    use hyper::body::Buf;
    use hyper::body::Body;
    use sqlx::mysql::MySqlPool;
    use std::env;
    use serde::{Deserialize, Serialize};
    use super::*;
    #[derive(Deserialize, Serialize, Debug)]
    struct Request{
        pub user_id_filter : u64 
    }
    impl Request {
        fn new(user_id_filter : u64 ) -> Self 
        {
            Self{
                user_id_filter
            }
        }
    }
    #[derive(Deserialize, Serialize)]
    pub struct Result{
        pub message : String,
        pub companies : Vec<Company>
    }
    pub async fn handler<'a>(
        body: Body,
        mut pool: sqlx::pool::PoolConnection<sqlx::mysql::MySql>,
    ) -> (String, u16)
    {
        // Aggregate the body...
    let whole_body = hyper::body::aggregate(body).await.unwrap();
    // Decode as JSON ...
    let mut data: Option<Request> = match serde_json::from_reader(whole_body.reader()) {
        Ok(data) => Some(data),
        _ => None,
    };
    println!("create company data req is : {:?}", data);
    match data {
        Some(data) => {
            if data.user_id_filter == 0 {
                // Select company
                match sqlx::query!(
                    "SELECT id,name, address, lat, lng, phone from company ",
                )
                .fetch_all(&mut pool)
                .await{
                    Err(_e) => {
                        println!("{:?}",_e);
                        (String::from("Internal Error"), 500)},
                    Ok(row) => {
                        let mut result = Result{
                            message : String::from("Success"),
                            companies : Vec::new(),
                         };
                        for row in row{
                            let new_company = Company::new(
                                row.id,
                                row.name, row.address, row.lat, row.lng, row.phone
                            );
                            result.companies.push(new_company);
                        }
                        (serde_json::to_string(&result).unwrap(), 200)
                }}
            }
            else {
                // Select company
                match sqlx::query!(
                    " SELECT company.*
                    FROM user_company 
                    LEFT JOIN company ON user_company.company_id = company.id 
                    WHERE user_company.user_id=?; ",
                    data.user_id_filter
                )
                .fetch_all(&mut pool)
                .await{
                    Err(_e) => {
                        println!("{:?}",_e);
                        (String::from("Internal Error"), 500)},
                    Ok(row) => {
                        let mut result = Result{
                            message : String::from("Success"),
                            companies : Vec::new(),
                         };
                        for row in row{
                            let new_company = Company::new(
                                row.id.unwrap(),
                                row.name.unwrap(), row.address.unwrap(), row.lat, row.lng, row.phone
                            );
                            result.companies.push(new_company);
                        }
                        (serde_json::to_string(&result).unwrap(), 200)
                }}
        }   }
        None => (String::from("Invalid Input"), 500),
    }
}
    #[cfg(test)]
    #[tokio::test]
    async fn test_create_company() {
        let database_url = env::var("DATABASE_URL").unwrap();
        let pool = MySqlPool::connect(&database_url[..]).await.unwrap();
        let request = Request::new(
            0
        );
        let req_body = Body::from(serde_json::to_string(&request).unwrap());
        let (message, status) = handler(req_body,pool.acquire().await.unwrap()).await;
        assert_eq!(status, 200);
        assert_eq!(message,"Success")
    }
}
