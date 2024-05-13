use mongodb::bson::{oid::ObjectId, DateTime, Bson};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub _id: Option<ObjectId>,
    pub user_id: Option<Bson>,
    pub admin: Option<bool>,
    pub banned: Option<bool>,
    pub username: String,
    pub email: Option<String>,
    pub passwordhash: String,
    pub server_timestamp: Option<DateTime>,
    pub user_timestamp: Option<String>,
}