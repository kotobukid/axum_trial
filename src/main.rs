use askama::Template;
use axum::{
    extract,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    routing::post,
    Router,
    extract::Query,
    extract::Form,
};
use std::net::SocketAddr;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
    where
        T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err)
            ).into_response()
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_page))
        .route("/hello", get(raw_string))
        .route("/name/:name/", get(template_page))
        .route("/g", get(catch_qs))
        .route("/p", post(catch_form))
        .route("/d", get(dynamic_file_reading))
        // .route("/p", get(catch_qs).post(catch_form))
        .fallback(handler_404)
        ;
    let port = 3000;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("listening on port {}", port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn raw_string() -> &'static str {
    "hello world(raw string)"
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    // #[serde(default, deserialize_with = "empty_string_as_none")]
    // foo: Option<i32>,
    name: Option<String>,
}

async fn catch_qs(Query(params): Query<Params>) -> String {
    println!("{:?}", &params.name);
    match &params.name {
        Some(s) => {
            // params.name.unwrap()
            String::from(s)
        }
        _ => { String::from("<None>") }
    }
}

async fn catch_form(Form(params): Form<Params>) -> String {
    println!("{:?}", &params.name);
    match &params.name {
        Some(s) => {
            String::from(s)
        }
        _ => { String::from("<None>") }
    }
}

async fn template_page(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    let template = HelloTemplate { name: String::from(name) };
    HtmlTemplate(template)
}

async fn index_page() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn dynamic_file_reading() -> impl IntoResponse {
    (StatusCode::OK, "dynamic file reading")
}