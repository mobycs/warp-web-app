mod prisma;
mod prisma_client;
mod handlers;
mod filters;

#[macro_use]
extern crate serde_json;

use warp::Filter;

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

    let api = warp::path!("api" / "v1")
        .and(filters::new_user())
        .or(filters::login_user());

    let routes = warp::any()
        .and(
            home
                .or(hello)
                .or(api)
                .or(filters::find_user_by_id())
        );

    // blazingly fast testing 
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
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