use std::sync::Arc;
use axum::{
    body::{boxed, Body, BoxBody},
    http::{Request, Response, StatusCode, Uri},
    Extension
};
use tower::ServiceExt;
use tower_http::services::ServeDir;
use crate::models::AppState;
use crate::filewriter::populate_html;

pub async fn file_handler(
    state: Extension<Arc<AppState>>,
    uri: Uri
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let mut uri_parts = uri.clone().into_parts();
    uri_parts.path_and_query = match uri.path() {
        "/" => Some("/home".parse().unwrap()),
        "" => Some("/home".parse().unwrap()),
        _ => Some(uri.path().parse().unwrap()),
    };
    let uri = Uri::from_parts(uri_parts).unwrap();
    let res = get_static_file(uri.clone()).await?;

    if let Err(err) = populate_html(uri.path()) {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR, 
            format!("There was a problem populating the page with fresh data. {}", err)
        ));
    };

    if res.status() == StatusCode::NOT_FOUND {
        match format!("{}.html", uri).parse() {
            Ok(uri_html) => get_static_file(uri_html).await,
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
        }
    } else {
        Ok(res)
    }
}

async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();
    match ServeDir::new("./generated").oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}
