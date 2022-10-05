use crate::prisma::user::{id, created_at};
use crate::prisma_client::get_prisma;
use crate::prisma::{user, PrismaClient};
use crate::filters::User;

use chrono::*;
use std::convert::Infallible;

pub async fn hello_user(name: String) -> Result<impl warp::Reply, Infallible> {
    let user_data: Option<user::Data> = get_prisma()
        .await
        .unwrap()
        .user()
        .find_first(vec![
            user::username::equals(name.clone())
        ])
        .exec()
        .await
        .unwrap();

    match user_data {
        Some(user) => Ok(format!("Hello, {}!", user.username)),
        None => Ok(format!("Hello, {}!\nIt appears to be your first time here!", name))
    }
}

pub async fn login_user(_new_user: User) -> Result<impl warp::Reply, Infallible> {
   Ok(warp::reply())
}

pub async fn create_user(new_user: User) -> Result<impl warp::Reply, Infallible> {
    if let Some(client) = get_prisma().await {
        match client
                .user()
                .create(
                    new_user.username,
                    vec![
                        user::password::set(
                            Some(
                                bcrypt::hash_with_result(new_user.password, bcrypt::DEFAULT_COST).unwrap().to_string() 
                            )
                        )
                    ]
                )
                .exec()
                .await {
                    Ok(_user) => warp::reply::json(&_user),
                    Err(err) => panic!("{:?}", err)
                };
    }

    Ok(format!("Ok"))
}

pub async fn find_user_by_id(id: i32) -> Result<impl warp::Reply, Infallible> {
    let prisma_client: Option<&PrismaClient> = get_prisma().await;

    match prisma_client
        .unwrap()
        .user()
        .find_unique(user::id::equals(id))
        .exec()
        .await {
            Ok(Some(found_user)) => Ok(
                format!(
                    "Welcome to {}'s profile!\nThis user joined {}", 
                    found_user.username, 
                    found_user.created_at.format("%e %B %Y")
                )
            ),
            Ok(None) => Ok(format!("bro where u find dat gui")),
            Err(err) => panic!("ERROR!!: {:?}", err),
        }
}