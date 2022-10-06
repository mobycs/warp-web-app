use warp::{Filter, Reply, filters::BoxedFilter};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String
}

fn json_body() -> impl Filter<Extract = (User,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn new_user() -> BoxedFilter<(impl Reply,)> {
    warp::path!("users" / "create")
        .and(warp::post())
        .and(json_body())
        .and_then(crate::handlers::create_user)
        .boxed()
}

pub fn login_user() -> BoxedFilter<(impl Reply,)> {    
    warp::path!("users" / "login")
        .and(warp::post())
        .and(json_body())
        .and_then(crate::handlers::login_user)
        .boxed()
}