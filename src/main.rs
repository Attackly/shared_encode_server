mod models;
pub mod schema;

use std::env;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use diesel::insert_into;
use models::result::Result;
use crate::models::reencodes::ReencodeRequest;
use uuid::Uuid;

// Diesel Async imports
use diesel_async::pg::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::RunQueryDsl;

pub type DbPool = Pool<AsyncPgConnection>;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_pool = establish_connection_pool();

    let state = AppState { db_pool };

    let app = Router::new()
        .route("/", get(|| async { "Server is running" }))
        .route("/new_entry", post(new_entry))
        .route("/test_v7", get(test_v7))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4321").await.unwrap();
    println!("Listening on http://localhost:4321");
    axum::serve(listener, app).await.unwrap();
}

pub fn establish_connection_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    Pool::builder(config)
        .build()
        .expect("Failed to create async database pool.")
}

#[axum::debug_handler]
async fn new_entry(Json(body): Json<ReencodeRequest> ) -> (StatusCode, Json<Result>) {    
    println!("Received new entry: {:?}", body);


    insert_into(crate::schema::reencodes::table)
        .values(body.into_reencode())
        .execute(&mut establish_connection_pool().get().await.unwrap())
        .await
        .expect("Error inserting new entry");

    println!("Inserted new entry into database");
    
    (StatusCode::CREATED, Json(Result { id: Uuid::now_v7() }))
}


async fn test_v7() -> (StatusCode, Json<Result>) {
    (StatusCode::CREATED, Json(Result { id: Uuid::now_v7() }))
}
