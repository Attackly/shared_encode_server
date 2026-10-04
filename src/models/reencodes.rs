use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Debug, Deserialize, Serialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::reencodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reencode {
    pub id: Uuid,
    pub name: String,
    pub hash: i64,
    pub ffmpeg_version: String,
    pub ffmpeg_command: String,
    pub svtav1_version: String,
    pub timestamp_start: DateTime<Utc>,
    pub timestamp_end: Option<DateTime<Utc>>
}


#[derive(Debug, Deserialize, Serialize,Insertable)]
#[diesel(table_name = crate::schema::reencodes)]
pub struct ReencodeRequest {
    pub hash: i64,
    pub ffmpeg_version: String,
    pub ffmpeg_command: String,
    pub svtav1_version: String

    // What about a Filesize? 
}

impl ReencodeRequest {
    pub fn into_reencode(self) -> Reencode {
        return Reencode {
            id: Uuid::now_v7(),
            name: "".to_string(),
            hash: self.hash,
            ffmpeg_version: self.ffmpeg_version.clone(),
            ffmpeg_command: self.ffmpeg_command.clone(),
            svtav1_version: self.svtav1_version.clone(),
            timestamp_start: Utc::now(),
            timestamp_end: None,
        };

    }
}