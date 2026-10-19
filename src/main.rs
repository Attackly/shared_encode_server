mod models;
pub mod schema;

use std::env;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use diesel::{ExpressionMethods, insert_into, query_dsl::methods::{FilterDsl, SelectDsl}};
use models::result::Result;
use crate::{models::{reencode_query::ReencodeQuery, reencodes::{self, ReencodeRequest}, vmaf_scores}, schema::reencodes::hash};
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

    // Test Area
    println!("FFmpeg version: {}", get_svtav1_version().await.unwrap());

    panic!("Test panic to check if the server is running");

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



async fn query_reencode_by_vmaf(Json(body): Json<ReencodeQuery>) {
    //TODO: THis needs to filter the VMAF Scores too for an avg (or Median? Maybe something weighted? from the Other Table)
    crate::schema::reencodes::table.filter(hash.eq(body.hash)).filter().get_result::<reencodes::Reencode>(&mut establish_connection_pool().get().await.unwrap());


}






// TODO: Useless for the Server Implementation.
async fn get_ffmpeg_version() -> std::result::Result<String, Box<dyn std::error::Error>> {
    let output = tokio::process::Command::new("ffmpeg")
        .arg("-version")
        .output()
        .await.unwrap();

    if output.status.success() {
        let version_info = String::from_utf8_lossy(&output.stdout);
        let first_line = version_info.lines().next().unwrap_or("");
        Ok(first_line.to_string())
    } else {
        Err(format!("Failed to get ffmpeg version: {}", String::from_utf8_lossy(&output.stderr)).into())
    }
}

async fn get_svtav1_version() -> std::result::Result<String, Box<dyn std::error::Error>> {

    //TODO: Hier muss noch die SVTAV1 version entnommen werden.
    let output = tokio::process::Command::new("ffmpeg")
    .args([
        "-f", "lavfi",
        "-i", "testsrc=duration=0.1:size=16x16",
        "-c:v", "libsvtav1",
        "-frames:v", "1",
        "-f", "null",
        "-"])
        .output()
        .await;

    println!("Output: {:?}", output);

    let output_unwrapped = output.unwrap();

    if output_unwrapped.status.success() {
        let version_info = String::from_utf8_lossy(&output_unwrapped.stdout);
        let first_line = version_info.lines().next().unwrap_or("");
        Ok(first_line.to_string())
    } else {
        Err(format!("Failed to get svt-av1 version: {}", String::from_utf8_lossy(&output_unwrapped.stderr)).into())
    }
}