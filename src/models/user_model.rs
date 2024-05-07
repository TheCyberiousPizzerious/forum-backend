use mongodb::{bson::oid::ObjectId, bson::DateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub _id: ObjectId,
    pub user_id: Uuid,
    pub admin: bool,
    pub banned: bool,
    pub username: String,
    pub email: String,
    pub passwordhash: String,
    pub server_timestamp: DateTime,
    pub user_timestamp: String,
}