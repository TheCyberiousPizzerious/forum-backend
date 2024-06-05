use crate::models::user_model::UserId;

use bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::user_model::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct Board {
    pub oid: Option<ObjectId>,
    pub board_id: String,
    //pub threads: Vec<String>,
    pub headers: String,
    pub content: String,
    pub newest_post: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Thread {
    pub _id: Option<ObjectId>,
    pub thread_id: Option<UserId>,
    pub user_id: UserId,
    pub username: String,
    pub board_id: String, // What board does this belong to 
    pub header: String,
    pub content: String,
    //pub posts: Vec<String>,
    pub timestamp: DateTime,
    pub user_timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub _id: Option<ObjectId>,
    pub user_id: UserId,
    pub post_id: UserId,
    pub username: String,
    pub thread_id: Option<UserId>, // what thread does this post belong to
    pub replying_to: Option<Vec<String>>,
    pub content: String,
    pub user_timestamp: String,
    pub timestamp: DateTime,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct SpecialPostRelInfoRec {
    pub board_id: String,
    pub thread_id: Option<UserId>,
    pub replying_to: Option<Vec<String>>,
    pub user_id: UserId,
    pub username: String,
    pub header: Option<String>,
    pub content: String,
    pub user_timestamp: String,
}