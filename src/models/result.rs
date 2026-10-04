use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Result {
    pub id: Uuid,
}