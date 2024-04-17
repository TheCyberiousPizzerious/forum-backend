use crate::models::message_model::MessageMessage;
use crate::models::{user_model::User, message_model::ErrorMessage}; 
use crate::utils::utils::{to_json, bson_now};

use actix_web::{delete, get, post, put, HttpResponse, web::{Data, Json}};
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
    // This can be made into a function!
    let jsonized_data = to_json(&data).unwrap();
    let bsonized_data = bson::to_bson(&jsonized_data).unwrap();
    if let Bson::Document(document) = bsonized_data {
        match collection.insert_one(document, None).await {
            Ok(_) => {
                let message = MessageMessage {message: data.user_id.to_string(),};
                HttpResponse::Ok().json(message)
            } 
            Err(e) => {
                let error = ErrorMessage {error: e.to_string(),};
                HttpResponse::InternalServerError().json(error)
            },
        };
    } else {

    }
    //client.database("userStorage");
    // Burde kalle på en login funksjon og logge inn for brukeren
    HttpResponse::Ok().json("")
}