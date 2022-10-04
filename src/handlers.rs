use crate::prisma_client::get_prisma;
use crate::prisma::{user};
use crate::User;
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