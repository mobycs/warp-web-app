use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::prisma_client;
use crate::prisma;
use crate::AppProps;

pub fn app<'a>(cx: Scope<AppProps>) -> Element {
    let routes = use_segment(&cx, || {
        Segment::new()
            .index(Home as Component)
            .fixed(
                "users",
                Route::new(()).nested(
                    Segment::new()
                        .index(UserList as Component)
                        .parameter(("id", User as Component)),

                ),
            )
            .fallback(NotFound as Component)
    });

    println!("{}", cx.props.initial_path);

    cx.render(rsx! {
        Router {
            initial_path: cx.props.initial_path.clone()
            routes: routes.clone()

            ul {
                Link { target: "/", li { "Home" }}
                Link { target: "/users" li { "Users" }}
                Link { target: "/users/1" li { "moby's profile" }}
            }


            Outlet {}
        }

    })
}

struct UserData {
    username: String,
    join_date: String
}


fn UserList(cx: Scope) -> Element {
    cx.render(rsx!(
        h1 {
            "dangitt"
        }
    ))
}
fn NotFound(cx: Scope) -> Element {
    cx.render(rsx!(
        h1 {
            "not found"
        }
    ))
}

fn User(cx: Scope) -> Element {
    let route = use_route(&cx)?;
    let id = route.parameters.get("id").expect("User ID not found");

    let user_id = id.parse::<i32>().unwrap();

    let user = use_future(&cx, &user_id, |_| async move {
        if let Some(client) = prisma_client::get_prisma().await {
            match client
                .user()
                .find_unique(
                    prisma::user::id::equals(user_id)
                )
                .exec()
                .await {
                    Ok(result) => match result {
                        Some(user) => Some(UserData {
                            username: user.username,
                            join_date: format!("{}", user.created_at.format("%e %B %Y"))
                        }),
                        None => None
                    }
                    Err(err) => panic!("{:?}", err),
                }
        } else {
            None
        }
    });

    cx.render(match user.value() {
        Some(Some(hi)) => rsx!(
            h1 {
                "{hi.username}"
            }

            p {
                "Joined {hi.join_date}"
            }
        ),
        Some(None) => rsx!(
            NotFound {}
        ),
        None => rsx!(
            h1 {
                "loading..."
            }
        )
    })
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx!(
        head {
            link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" }
        }

        h1 {
            "hello"
        }
    ))
}

/*
let prisma_client: Option<&PrismaClient> = get_prisma().await;
    
    let user_data: ProfileProps = match prisma_client
        .unwrap()
        .user()
        .find_unique(user::id::equals(id))
        .exec()
        .await {
            Ok(Some(found_user)) => ProfileProps {
                username: found_user.username, 
                date_joined: format!("{}", found_user.created_at.format("%e %B %y"))  
            },
            Ok(None) => ProfileProps {
                username: "User does not exist".to_string(), date_joined: "".to_string()
            },
            Err(e) => panic!("An error has occured: {:?}", e),
        };

    let mut vdom: VirtualDom = VirtualDom::new_with_props(app, user_data);
    let _ = vdom.rebuild();
    Ok(warp::reply::html(dioxus::ssr::render_vdom(&vdom)))
}
*/