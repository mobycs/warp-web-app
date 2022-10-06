use crate::prisma_client::get_prisma;
use crate::prisma::{user};
use crate::filters::User;


use std::convert::Infallible;
//use chrono::*;
//use serde_json::json;

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