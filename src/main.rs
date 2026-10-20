mod models;
pub mod schema;

use std::{collections::HashMap, env};
use axum::{
    Json, Router, extract::Query, http::StatusCode, routing::{get, post},
};
use diesel::{ExpressionMethods, QueryDsl, insert_into, query_dsl::methods::FilterDsl};
use models::result::Result;
use crate::models::{reencode_query::ReencodeQuery, reencodes::{self, Reencode, ReencodeRequest}};
use uuid::Uuid;

// Diesel Async imports
use diesel_async::pg::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::RunQueryDsl;
use diesel::prelude::*;

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
        .route("/query_vmaf", get(query_reencode_by_vmaf))
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



#[axum::debug_handler]
async fn query_reencode_by_vmaf(Query(params) : Query<HashMap<String ,String>>) -> (StatusCode, Json<Reencode>) {
    //TODO: THis needs to filter the VMAF Scores too for an avg (or Median? Maybe something weighted? from the Other Table)
    let mut conn = establish_connection_pool().get().await.unwrap();

    let query = crate::schema::reencodes::table.inner_join(crate::schema::vmaf_scores::table);


    // TODO: This unwrap parse unwrap is a warcrime
    let result: reencodes::Reencode = FilterDsl::filter(query, crate::schema::reencodes::hash.eq(params.get("hash").unwrap().parse::<i64>().unwrap()))
        .select(crate::schema::reencodes::all_columns) // Use reencodes::all_columns instead of reencodes::table
        .get_result(&mut conn)
        .await.unwrap();


    println!("The Result: {:?}", result);

    return (StatusCode::OK, Json(result));
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

// TODO: Useless for Server
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