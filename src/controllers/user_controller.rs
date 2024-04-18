use crate::models::message_model::{MessageMessage, MessageTraits};
use crate::models::{user_model::User, message_model::ErrorMessage}; 
use crate::utils::utils::{to_json, bson_now};

use actix_web::{delete, get, post, put, HttpResponse, web::{Data, Json}};
use bson::to_bson;
use mongodb::{bson::{Document, oid::ObjectId, Bson}, Collection};
use std::sync::Arc;
use uuid::Uuid;

#[post("/register")]
pub async fn register_user(collection: Data<Arc<Collection<Document>>>, new_user: Json<User>) -> HttpResponse {
    println!("I GOT YOUR REQUEST");
    let data = User {
        _id: ObjectId::new(),
        user_id: Uuid::new_v4(),
        admin: false,
        username: new_user.username.to_owned(),
        email: new_user.email.to_owned(),
        passwordhash: new_user.passwordhash.to_owned(),
        server_timestamp: bson_now(), // Server timestamp
        user_timestamp: new_user.user_timestamp.to_owned(),
    };

    let json_data = to_json(&data).unwrap();
    let bson_data = to_bson(&json_data).unwrap();
    if let Bson::Document(document) = bson_data {
        match collection.insert_one(document, None).await {
            Ok(_) => HttpResponse::Ok().json(ErrorMessage::new_from(data.user_id.to_string())),
            Err(e) => HttpResponse::InternalServerError().json(ErrorMessage::new_from(e.to_string())),
        }
    } else {
        HttpResponse::InternalServerError().json(ErrorMessage::new_from("Error occured internally converting objects between types".to_string()))
    }
    // every handeling of json to bson to document needs to be moved here

    //client.database("userStorage");
    // Burde kalle på en login funksjon og logge inn for brukeren
}