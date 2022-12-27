use askama::Template;
use axum::{
    // extract,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

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
        .route("/hello", get(raw_string))
        .route("/", get(index_page));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn raw_string() -> &'static str {
    "hello world(raw string)"
}

async fn index_page() -> impl IntoResponse {
    let template = HelloTemplate { name: "taro" };
    HtmlTemplate(template)
}