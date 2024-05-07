use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterLoginLog {
    pub _id: ObjectId,
    pub monitor_id: Uuid,
    pub user_id: Uuid,
    pub address: String,
    pub timestamp: DateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestLog {
    pub _id: ObjectId,
    pub monitor_id: Uuid,
    pub user_requesting: Uuid,
    pub address: String,
    pub logs_requested: String,
    pub timestamp: DateTime,
}

pub enum LogTypes {
    loginLogs,
    registerLogs,
}
