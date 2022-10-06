mod prisma;
mod prisma_client;
mod handlers;
mod filters;
mod app;

extern crate serde_json;
extern crate dioxus;
use dioxus::prelude::*;
use serde_json::json;

use warp::{Filter, path::FullPath};

#[derive(PartialEq, Props)]
pub struct AppProps {
    pub initial_path: String
}

#[tokio::main]
async fn main() {
    let app = warp::get()
        .and(warp::path::full())
        .map(|p: FullPath| {
            println!("{}", p.as_str());
            let mut vdom = VirtualDom::new_with_props::<AppProps>(app::app, AppProps { initial_path: p.as_str().to_string() });
            let _ = vdom.rebuild();
           
            warp::reply::html(dioxus_ssr::render_vdom(&vdom))
        });

    let api = warp::path!("api" / "v1")
        .and(filters::new_user())
        .or(filters::login_user());

    let routes = warp::any()
        .and(
            api
                .or(app)
        );

    let port: u16 = match std::env::var(
        "PORT"
    ) {
        Ok(x) => x.parse::<u16>().unwrap(),
        _ => 3030,
    };

    // blazingly fast testing 
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await; 
}

#[cfg(test)]
mod tests {
    use crate::filters;

    #[tokio::test]
    async fn test_new_user() {
//        let mut map = std::collections::HashMap::new();
//        map.insert("username", "moby");
//        map.insert("password", "johnnyoops123");


        let filter = filters::new_user();

        let res = warp::test::request()
                .method("POST")
                .path("/users/create")
                .json(
                    &json!({
                        "username": "moby",
                        "password": "mr_wrldwidejorgerodriguezgamer23ayy"
                    })
                )
                .reply(&filter)
                .await;

        println!("{:?}", res);
        assert_eq!(res.status(), 200);
    }
}