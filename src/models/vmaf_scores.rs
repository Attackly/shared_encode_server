use serde::{Deserialize, Serialize};
use diesel::{Selectable, associations::{Associations, Identifiable}, deserialize::{Queryable}, prelude::Insertable};
use uuid::Uuid;

use crate::models::reencodes::Reencode;


#[derive(Debug, Deserialize, Serialize, Insertable, Queryable, Selectable, Associations, Identifiable)]
#[diesel(belongs_to(Reencode))]
#[diesel(table_name = crate::schema::vmaf_scores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VmafScore {
    pub id: Uuid,
    pub reencode_id: Uuid,
    pub mean: f64,
    pub harmonic_mean: f64,
    pub min: f64,
    pub percentile_1: f64,
    pub percentile_5: f64,
    pub max: f64,
}

//TODO: We will need the VMAF model here too...
// Maybe write the used reencode ID into the Container so we can use a command like 
// shared-encode upvote ./rencoded-video.mkv
// shared-encode downvote ./rencoded-video.mkv