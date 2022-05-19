use serde::{Serialize,Deserializa};

#[derive(Deserialize, Serialize, Debug)]
enum UserStatus{
    Blocked,
    Created,
    Hidden,
    Completed
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User{
    id : u64,
    username       : String, 
    fullname       : String, 
    family         : String, 
    email          : String, 
    image          : String, 
    status         : UserStatus,
    created_at  : String
}
impl User {
    pub fn new(
        id : u64,
        username: String,
        fullname: String,
        family: String, 
        email: String, 
        image: String,
        status: UserStatus,
        created_at: String
    )-> Self {
        Self {
            id ,
            username,
            fullname,
            family, 
            email, 
            image,
            status,
            created_at
        }
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
        pub users : Vec<Company>
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
    println!("create users data req is : {:?}", data);
    match data {
        Some(data) => {
                // Select company
                match sqlx::query!(
                    "SELECT id,username, fullname, family, email, image, status, created_at from user ",
                )
                .fetch_all(&mut pool)
                .await{
                    Err(_e) => {
                        println!("{:?}",_e);
                        (String::from("Internal Error"), 500)},
                    Ok(row) => {
                        let mut result = Result{
                            message : String::from("Success"),
                            users : Vec::new(),
                         };
                        for row in row{
                            let new_user = User::new(
                                row.id,
                                row.username,
                                row.fullname, 
                                row.family,
                                row.email, 
                                row.image,
                                row.status,
                                row.created_at
                            );
                            result.users.push(new_user);
                        }
                        (serde_json::to_string(&result).unwrap(), 200)
                }
            }
        }   
        None => (String::from("Invalid Input"), 500),
    }

}
    
}