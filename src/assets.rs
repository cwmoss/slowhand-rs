use super::AppState;
use crate::dataset::{self, Dataset};
use crate::store::Doc;
use anyhow::Result;
use axum::{
    body::{Body, Bytes},
    extract::{DefaultBodyLimit, FromRequest, Multipart, Path, Query, Request, State},
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Json, Redirect, Response},
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn get_routes() -> Router<Arc<AppState>> {
    let r = Router::new()
        .route("/images/{dsname}", post(add_image))
        .route("/files/{dsname}", post(add_file));
    r
}

struct ImageAsset {
    _id: String,
    _type: String,
    asset_id: String,
    extension: String,
    mime_type: String,
    original_filename: String,
    path: String,
    upload_id: String,
    size: usize,
    sha1hash: String,
    url: String,
    width: usize,
    height: usize,
}

impl ImageAsset {
    pub fn new_from_upload(
        pname: &String,
        hash: &String,
        extension: &String,
        mime_type: &String,
        size: usize,
        orig_name: String,
        width: usize,
        height: usize,
    ) -> Self {
        let path = format!("{}-{}x{}.{}", hash, width, height, extension);
        Self {
            _id: hash.to_string(),
            _type: "_sh.asset".to_string(),
            asset_id: format!("{}-{}x{}-{}", hash, width, height, extension),
            extension: extension.clone(),
            mime_type: mime_type.clone(),
            original_filename: orig_name,
            path: path.clone(),
            upload_id: "".to_string(),
            size,
            sha1hash: hash.to_string(),
            url: format!("/images/{}/{}", pname, path),
            width,
            height,
        }
    }
}

/*

>> upload received

    - get size
    - get mime-type and extension
    - TODO: validate allowed files, sizes, etc...
    - get unique id => sha1_sum
    - get image dimensions
    - TODO update image type???
    - make:
        * id: hash "-" w "x" h "-" extension
            949aff283c8b68292672a61221ec0b018a526c17-800x600-jpg
        * path: hash "-" w "x" h "." extension
            949aff283c8b68292672a61221ec0b018a526c17-800x600.jpg
    - move file to @var/assets/PROJECT-NAME/PATH
    - make asset-document
    - create or update asset-document

>> asset-document

*/
async fn add_image(
    State(app_state): State<Arc<AppState>>,
    Path(dsname): Path<String>,
    Query(params): Query<std::collections::HashMap<String, String>>,
    upl: Upload,
) -> impl IntoResponse {
    let ds = dataset::Dataset::load(dsname, &app_state.conf.projects, &app_state.conf.var).await;
    let name = if let Some(name) = params.get("filename") {
        name.to_string()
    } else {
        "".to_string()
    };
    let size = upl.0.len();
    println!("multipart upload received, size: {}", size);

    let (mimetype, ext) = get_mime_type(&upl.0);
    let (w, h) = get_image_size(&upl.0).unwrap();
    let hash = {
        let mut m = sha1_smol::Sha1::new();
        m.update(&upl.0);
        m.digest().to_string()
    };
    let asset = ImageAsset::new_from_upload(&ds.name, &hash, &ext, &mimetype, size, name, w, h);
    let dest = ds.assets.join(asset.path);
    tokio::fs::write(dest, upl.0).await.unwrap();
    println!("saved to mp-upload.jpg {} {} {}", upl.1, mimetype, ext);
    (StatusCode::OK, "image uploaded".to_string())
}

async fn add_file(
    State(app_state): State<Arc<AppState>>,
    Path(album): Path<String>,
    Query(params): Query<std::collections::HashMap<String, String>>,
    upl: Upload,
) -> impl IntoResponse {
    println!("multipart upload received, size: {}", upl.0.len());
    tokio::fs::write("mp-upload.jpg", upl.0).await.unwrap();
    println!("saved to mp-upload.jpg {}", upl.1);
    (StatusCode::OK, "file uploaded".to_string())
}

// https://docs.rs/imagesize/0.14.0/imagesize/
// alt: https://github.com/xiaozhuai/imageinfo-rs
pub fn get_image_size(buf: &[u8]) -> Result<(usize, usize)> {
    let size = imagesize::blob_size(&buf)?;
    Ok((size.width, size.height))
}

pub fn get_mime_type(buf: &[u8]) -> (String, String) {
    // let buf = [0xFF, 0xD8, 0xFF, 0xAA];
    if let Some(kind) = infer::get(&buf) {
        (kind.mime_type().to_string(), kind.extension().to_string())
    } else {
        ("application/octetstream".to_string(), "".to_string())
    }
    // assert_eq!(kind.mime_type(), "image/jpeg");
    // assert_eq!(kind.extension(), "jpg");
}

struct Upload(Bytes, String);

impl<S> FromRequest<S> for Upload
where
    Bytes: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        println!("FromRequest Jpeg/multipart\n");
        let Some(content_type) = req.headers().get(header::CONTENT_TYPE) else {
            return Err(StatusCode::BAD_REQUEST);
        };
        let Ok(content_type) = content_type.to_str() else {
            return Err(StatusCode::BAD_REQUEST);
        };
        dbg!(content_type);

        // we take the first part only for simplicity
        // only support filepond (2 times name filepond)
        if content_type.starts_with("multipart/form-data") {
            return Self::handle_multipart(
                Multipart::from_request(req, state)
                    .await
                    .map_err(|_| StatusCode::BAD_REQUEST)?,
            )
            .await;
            // data
        }
        // TODO: other types
        let body = if content_type == "image/jpeg" {
            (
                Bytes::from_request(req, state)
                    .await
                    .map_err(|_| StatusCode::BAD_REQUEST)?,
                "meta".to_string(),
            )
        } else {
            return Err(StatusCode::BAD_REQUEST);
        };

        Ok(Self(body.0, body.1))
    }
}

impl Upload {
    async fn handle_multipart(mut multipart: Multipart) -> Result<Self, StatusCode> {
        // dbg!(multipart);

        let Ok(Some(field)) = multipart.next_field().await else {
            return Err(StatusCode::BAD_REQUEST);
        };
        let name = field.name().unwrap().to_string();
        // dbg!(name);

        // TODO
        // https://docs.rs/axum/latest/axum/extract/multipart/struct.Field.html
        let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
        match name.as_str() {
            "filepond" => {
                let Ok(Some(field)) = multipart.next_field().await else {
                    return Err(StatusCode::BAD_REQUEST);
                };
                // let fname = field.name().unwrap().to_string();
                // dbg!(name);
                let fdata = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                Ok(Self(fdata, format!("{:?}", data)))
            }
            _ => Ok(Self(data, "".to_string())),
        }
    }
}
