use axum::Router;
use axum::routing::*;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use app::*;
use leptos::logging::log;
use server::handlers::get_base_tags;
use server::handlers::login;
use server::handlers::register;
use server::handlers::set_base_tags;
use tower_http::cors::{Any, CorsLayer};

mod config;

use crate::config::*;
#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let db_pool = get_db_pool(&config.database_url).await;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/registration", post(register))
        .route("/login", post(login))
        .route("/get_base_tags", get(get_base_tags))
        .route("/set_base_tags", post(set_base_tags))
        .with_state(db_pool)
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
            );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
