use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

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
    pub oid: Option<ObjectId>,
    pub thread_id: String,
    pub user_id: String,
    pub board_id: String, // What board does this belong to 
    pub header: String,
    pub content: String,
    //pub posts: Vec<String>,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    pub oid: Option<ObjectId>,
    pub post_id: String,
    pub user_id: String,
    pub thread_id: String, // what thread does this post belong to
    pub replying_to: Vec<String>,
    pub content: String,
    pub timestamp: String,
}