use mongodb::bson::{oid::ObjectId, DateTime, Bson};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterLoginLog {
    pub _id: ObjectId,
    pub monitor_id: Bson,
    pub user_id: Bson,
    pub address: String,
    pub timestamp: DateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestLog {
    pub _id: ObjectId,
    pub monitor_id: Bson,
    pub user_requesting: Bson,
    pub address: String,
    pub logs_requested: String,
    pub timestamp: DateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginLog {
    pub _id: ObjectId,
    pub monitor_id: Bson,
    pub user_id: Bson,
    pub address: String,
    pub is_succesfull: bool,
    pub timestamp: DateTime,
}


/*
pub enum LogTypes {
    loginLogs,
    registerLogs,
}
*/
