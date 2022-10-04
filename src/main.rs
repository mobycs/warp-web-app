mod prisma;
mod prisma_client;
mod handlers;

use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Deserialize, Serialize)]
pub struct User {
    username: String,
    password: String
}

fn json_body() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let home = warp::path::end()
        .and(warp::get())
        .map(|| {
            format!("Welcome home!")
        });

    let hello = warp::path!("hello" / String)
        .and(warp::get())
        .and_then(handlers::hello_user);

    let new_user = warp::path!("users" / "create")
        .and(warp::post())
        .and(json_body())
        .and_then(handlers::create_user);

    let api = warp::path!("api" / "v1")
        .and(new_user);


    let routes = warp::any()
        .and(
            home
                .or(hello)
                .or(api)
        );

    // blazingly fast testing 
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await; 
}

#[tokio::test]
async fn test_new_user() -> Result<(), String> {
    let mut map = std::collections::HashMap::new();
    map.insert("username", "moby");
    map.insert("password", "johnnyoops123");

    let client = reqwest::Client::new();
    match client.post("127.0.0.1:3030/api/v1/users/create")
        .json(&map)
        .send()
        .await? {
            Ok(res) => {
                Ok(())
            },
            Err(err) => Err(format!("An error has occurred: {:?}", err))
    }
}