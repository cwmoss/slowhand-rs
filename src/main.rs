pub mod config;
pub mod dataset;
pub mod dataset_handler;
pub mod gql;
pub mod schema;
pub mod schema_kdl;
pub mod store;

use std::collections::HashMap;
use std::time::Instant;
use turso::Builder;

use crate::{config::Config, dataset::Dataset, schema::Schema, store::Doc};
use axum::{
    body::{Body, Bytes},
    extract::{DefaultBodyLimit, FromRequest, Multipart, Path, Query, Request, State},
    http::{header, HeaderValue, StatusCode},
    middleware,
    response::{Html, IntoResponse, Json, Redirect, Response},
    routing::{get, post},
    serve::Listener,
    Router,
};
use rust_embed::Embed;
use serde_json::json;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub conf: Config,
    // pub datasets: HashMap<String, Dataset>,
    pub system: Dataset,
    pub start_time: Instant,
}

// https://github.com/tokio-rs/axum/blob/main/examples/key-value-store/src/main.rs
// https://oneuptime.com/blog/post/2026-01-07-rust-axum-rest-api/view
// https://docs.rs/axum/latest/axum/attr.debug_handler.html
// https://docs.rs/axum/latest/axum/extract/struct.State.html
// type SharedState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conf = Config::new();
    let open_browser = false;
    let bind_host = "127.0.0.1";
    let port = 10365;
    // let http_prefix = format!("{}/", args.prefix.trim_end_matches('/'));
    let http_prefix = "/";
    let hostport = format!("{}:{}", bind_host, port).to_string();
    conf.setup();
    let app_state = AppState {
        system: conf.system_dataset().await,
        conf,
        // datasets: HashMap::new(),
        start_time: Instant::now(),
    };
    let state = Arc::new(app_state);
    // let data_router = Router::new().route("/schema/{dsname}", get(show_schema));
    let data_router = dataset_handler::get_routes();
    // .route("/{*subpath}", get(show_subpath))
    // .route("/", get(show_album))
    // .route("/", post(upload_image_maybe_multipart))
    //.layer(DefaultBodyLimit::max(10 * 1024 * 1024));

    let router = Router::new()
        .nest("/data", data_router)
        .layer(axum_server_timing::ServerTimingLayer::new("HelloService"))
        // .route("/_assets/{*file}", get(static_handler))
        // .route("/stats", get(stats_handler))
        // .route("/", get(if_single_album_redirect))
        .route("/favicon.ico", get(favicon))
        .with_state(state)
        .fallback_service(get(not_found));

    let router = match String::from(http_prefix.clone()).as_str() {
        "/" | "" => router,
        http_prefix => Router::new().nest(&http_prefix, router),
    };

    // start the server
    let listener = tokio::net::TcpListener::bind(hostport.clone()).await;
    let listener = match listener {
        Ok(l) => l,
        Err(msg) => {
            println!("unable to bind. trying different port ({})", msg);
            let hostport = format!("{}:0", bind_host);
            tokio::net::TcpListener::bind(hostport.clone())
                .await
                .unwrap()
        }
    };

    println!(
        "Listening on http://{:?}{}",
        listener.local_addr().ok().unwrap(),
        http_prefix
    );

    if open_browser {
        // let future = after_start(hostport.clone());
        // set_timeout_async!(future, 600);
    }
    axum::serve(listener, router).await.unwrap();
    Ok(())
}

/*
$app->get("/data/index", api\query::class);

$app->get("/data/doc/{dsname}/{id}", api\doc::class);
$app->get("/data/query/{dsname}", api\query::class);
$app->get("/data/count/{dsname}", api\query::class);
$app->get("/data/search/{dsname}", api\query::class);
$app->get("/data/info/{dsname}", api\query::class);
$app->get("/data/schema/{dsname}", api\query::class);


$app->post("/data/mutate/push/{dsname}", api\mutate::class);
$app->post("/data/mutate/{dsname}", api\mutate::class);
*/
async fn not_found() -> Html<&'static str> {
    Html("<h1>404</h1><p>Not Found</p>")
}

/*

https://kdl.dev/play/
https://crates.io/crates/knus/
https://github.com/bearcove/styx


*/

#[derive(Embed)]
#[folder = "public/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}

async fn favicon() -> impl IntoResponse {
    StaticFile("favicon.ico")
}

async fn static_handler(Path(path): Path<String>) -> impl IntoResponse {
    StaticFile(path)
}
