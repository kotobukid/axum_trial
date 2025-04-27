use askama::Template;
use axum::http::HeaderValue;
use axum::{
    extract,
    extract::Form,
    // Json,
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    routing::post,
    Router,
};
use hyper::body::Body;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;

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
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_page))
        .route("/hello", get(raw_string))
        .route("/name/{name}/", get(template_page))
        .route("/g", get(catch_qs))
        .route("/p", post(catch_form))
        .route("/d.html", get(dynamic_file_reading))
        .route("/json", get(json_sample))
        .route("/api/name", get(json_sample))
        // .route("/p", get(catch_qs).post(catch_form))
        .fallback(handler_404);
    let port = 3000;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("listening on port {}", port);

    axum::serve(listener, app).await.unwrap();
}

async fn raw_string() -> &'static str {
    "hello world(raw string)"
}

#[derive(Debug, Deserialize, Serialize)]
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
        _ => String::from("<None>"),
    }
}

async fn catch_form(Form(params): Form<Params>) -> String {
    println!("{:?}", &params.name);
    match &params.name {
        Some(s) => String::from(s),
        _ => String::from("<None>"),
    }
}

async fn template_page(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    let template = HelloTemplate {
        name: String::from(name),
    };
    HtmlTemplate(template)
}

async fn index_page() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn dynamic_file_reading() -> Html<String> {
    let mut f = File::open("./templates/dynamic_hello.html").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    Html(contents)
    // (StatusCode::OK, &contents)
}

async fn json_sample(// pagination: Option<Query<Pagination>>,
    // State(db): State<Db>,
) -> impl IntoResponse {
    let d = Params {
        name: Some(String::from("hoge")),
    };

    let body = serde_json::to_string(&d).expect("Failed to serialize.");

    let mut response: Response<String> = Response::new(body.into());

    let headers = response.headers_mut();
    headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("POST, GET, PUT, DELETE, OPTIONS"),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("Content-Type"),
    );

    response
}
