use super::AppState;
use crate::dataset::{self, Dataset};
use crate::store::Doc;
use anyhow::Result;

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
// use futures::TryStreamExt;
use futures_util::{stream::TryStreamExt, StreamExt};
use serde_json::json;
use std::io;
use std::sync::Arc;
use tokio::io::AsyncBufReadExt;
use tokio_stream::wrappers::LinesStream;
use tokio_util::io::StreamReader;

pub fn get_routes() -> Router<Arc<AppState>> {
    let r = Router::new()
        .route("/schema/{dsname}", get(show_schema))
        .route("/doc/{dsname}/{id}", get(get_doc))
        .route("/mutate/push/{dsname}", post(push));
    r
}

async fn get_doc(
    State(app_state): State<Arc<AppState>>,
    Path((dsname, id)): Path<(String, String)>,
) -> impl axum::response::IntoResponse {
    // let ds = app_state.datasets.get(&dsname);
    let ds = dataset::Dataset::load(dsname, &app_state.conf.projects, &app_state.conf.var).await;

    return Json(ds.store.get_doc(id).await);
}

// https://github.com/tokio-rs/axum/blob/main/axum-extra/src/json_lines.rs#L111
// https://github.com/tokio-rs/axum/discussions/2506
async fn push(
    State(app_state): State<Arc<AppState>>,
    Path(dsname): Path<String>,
    body: axum::body::Body,
) -> impl axum::response::IntoResponse {
    // let ds = app_state.datasets.get(&dsname);
    let ds = dataset::Dataset::load(dsname, &app_state.conf.projects, &app_state.conf.var).await;

    // https://docs.rs/tokio-stream/latest/tokio_stream/wrappers/struct.LinesStream.html

    let stream = body.into_data_stream();
    let stream = stream.map_err(io::Error::other);
    //let stream = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
    let read = StreamReader::new(stream);
    let mut lines_stream = LinesStream::new(read.lines());
    //let lines_stream = LinesStream::new(stream.lines());

    let mut res: Vec<String> = [].to_vec();

    while let Some(line) = lines_stream.next().await {
        if let Some(l) = line.ok() {
            let doc: Doc = serde_json::from_str(&l).unwrap();
            let id = doc._id.clone();
            ds.store.create_or_replace(doc).await.unwrap();
            res.push(id);
        }
        // println!("{}", line?);
    }

    Json(json!(res))
}

async fn show_schema(
    State(app_state): State<Arc<AppState>>,
    Path(dsname): Path<String>,
) -> impl axum::response::IntoResponse {
    // let ds = app_state.datasets.get(&dsname);
    let ds = dataset::Dataset::load(dsname, &app_state.conf.projects, &app_state.conf.var).await;
    if let Some(schema) = ds.schema {
        return Json(json!(schema));
    }

    return Json(json!("Not found"));
}
