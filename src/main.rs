mod todo;
mod custom;

use std::{env, sync::Arc};

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router
};

use custom::middleware::request_logging_middleware;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use crate::todo::{
    handler::{self, Handler},
    repository::Repository
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let repo = Repository::new(pool);
    let handler = Arc::new(Handler::new(repo));

    let app = Router::new()
        .route("/todos", get(handler::get_all))
        .route("/todos", post(handler::create))
        .route("/todos/:id", get(handler::get_by_id))
        .route("/todos/:id", put(handler::update))
        .route("/todos/:id", delete(handler::delete))
        .layer(middleware::from_fn(request_logging_middleware))
        .with_state(handler);

    let addr = format!("0.0.0.0:{}", env::var("HTTP_PORT").expect("HTTP_PORT is not set"));

    log::info!("Listening on {}", &addr);
    let listener = TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
