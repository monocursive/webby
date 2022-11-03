use std::net::SocketAddr;

use axum::{
    body::Body,
    extract,
    extract::Query,
    response::Html,
    routing::{get, post, MethodRouter},
    Json, Router,
};

use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(greet())
        .merge(greet_enhanced())
        .merge(greet_json());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn route(path: &str, method_router: MethodRouter<Body>) -> Router {
    Router::new().route(path, method_router)
}

fn greet() -> Router {
    async fn handler() -> &'static str {
        "Hello you!"
    }
    route("/hello", get(handler))
}

#[derive(Deserialize)]
struct GreetParameter {
    name: String,
}

#[derive(Deserialize)]
struct CreateGreeting {
    name: String,
}
#[derive(Deserialize, Serialize)]
struct Greeting {
    name: String,
    hello: String,
}

fn greet_enhanced() -> Router {
    async fn handler(Query(params): Query<GreetParameter>) -> Html<String> {
        Html(format!("<h1>Hello {}</h1>", params.name))
    }
    route("/greet", get(handler))
}

fn greet_json() -> Router {
    async fn handler(extract::Json(payload): extract::Json<CreateGreeting>) -> Json<Greeting> {
        let greeting = Greeting {
            name: payload.name,
            hello: "world".to_string(),
        };
        Json(greeting)
    }
    route("/greet_json", post(handler))
}
