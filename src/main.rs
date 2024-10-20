#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

pub mod components;
pub mod pages;

use pages::home_page::HomePage;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    HomePage {},
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./assets/tailwind.css") }

        Router::<Route> {}
    }
}

// #[component]
// fn Blog(id: i32) -> Element {
//     rsx! {
//         Link { to: Route::Home {}, "Go to counter" }
//         "Blog post {id}"
//     }
// }

// #[component]
// fn Home() -> Element {
//     let mut count = use_signal(|| 0);
//     let mut text = use_signal(|| String::from("..."));

//     rsx! {
//         Link {
//             to: Route::Blog {
//                 id: count()
//             },
//             "Go to blog"
//         }
//         div {
//             h1 { "High-Five counter: {count}" }
//             button { onclick: move |_| count += 1, "Up high!" }
//             button { onclick: move |_| count -= 1, "Down low!" }
//             button {
//                 onclick: move |_| async move {
//                     if let Ok(data) = get_server_data().await {
//                         tracing::info!("Client received: {}", data);
//                         text.set(data.clone());
//                         post_server_data(data).await.unwrap();
//                     }
//                 },
//                 "Get Server Data"
//             }
//             p { "Server data: {text}"}
//         }
//     }
// }

// #[server(PostServerData)]
// async fn post_server_data(data: String) -> Result<(), ServerFnError> {
//     tracing::info!("Server received: {}", data);
//     Ok(())
// }

// #[server(GetServerData)]
// async fn get_server_data() -> Result<String, ServerFnError> {
//     Ok("Hello from the server!".to_string())
// }
