use std::sync::Arc;
use tera::{Context, Tera};
use axum::{routing::get, Router, http};
use sync_wrapper::SyncWrapper;
use axum::extract::Extension;
use axum::response::IntoResponse;
use axum::response::Html;
use hyper::Response;


async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn index(
    Extension(templates): Extension<Templates>,
) -> impl IntoResponse {
    Html(templates.render("index", &Context::new()).unwrap())
}

async fn styles() -> impl IntoResponse {
    Response::builder()
        .status(http::StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../public/styles.css").to_owned())
        .unwrap()
}


#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("base.html", include_str!("../template/base.html")),
        ("index", include_str!("../template/index.html")),
    ])
        .unwrap();

    let router = Router::new()
        .route("/", get(index))
        .route("/styles.css", get(styles))
        .layer(Extension(Arc::new(tera)));


    let sync_wrapper = SyncWrapper::new(router);
    Ok(sync_wrapper)
}