use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginMonitoring {
    pub oid: Option<ObjectId>,
    pub monitor_id: String,
    pub user_id: String,
    pub address: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterMonitoring {
    pub oid: Option<ObjectId>,
    pub monitor_id: String,
    pub user_id: String,
    pub address: String,
    pub timestamp: String,
}