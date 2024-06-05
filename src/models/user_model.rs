use core::fmt;

use mongodb::bson::{oid::ObjectId, DateTime, Bson};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub _id: Option<ObjectId>,
    pub user_id: Option<UserId>,
    pub admin: Option<bool>,
    pub banned: Option<bool>,
    pub username: String,
    pub email: Option<String>,
    pub passwordhash: String,
    pub server_timestamp: Option<DateTime>,
    pub user_timestamp: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum UserId {
    Uuid(Uuid),
    Bson(Bson),
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserId::Uuid(uuid) => write!(f, "{}", uuid),
            UserId::Bson(bson) => write!(f, "{}", bson),
        }
    }
}