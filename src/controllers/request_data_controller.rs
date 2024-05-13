use crate::models::{message_model::ErrorMessage, user_model::User};

use crate::models::message_model::{MessageMessage, MessageTraits};

use actix_web::http::header::Allow;
use actix_web::{
    get, web::{self, Data}, HttpResponse};
use mongodb::{bson::doc, options::FindOneOptions, Client};
use futures_util::StreamExt;
use std::sync::Arc;
use uuid::Uuid;

#[get("/requestUserId/{id}")]
pub async fn search_uuid(client: Data<Arc<Client>>, id: web::Path<String>) -> HttpResponse {
    println!("Someone requested a user");
    let requested_uuid = match Uuid::parse_str(&id) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Failed to parse requested id: {}", e);
            Uuid::nil()
        },
    };
    if requested_uuid.is_nil() {
        HttpResponse::BadRequest().json(ErrorMessage::new_from("The user id requested is invalid".to_string()))
    } else {
        let filter = doc! { "user_id": requested_uuid.to_string() };
        let find_options = FindOneOptions::builder().build();
        let result = client.database("userStorage").collection::<User>("users").find_one(filter, find_options).await;
        match result {
            Ok(val) => HttpResponse::Ok().json(val),
            Err(e) => HttpResponse::NotFound().json(ErrorMessage::new_from(e.to_string()))
        }
    }
}

#[get("/getAllUsers")]
pub async fn get_all_users(client: Data<Arc<Client>>) -> HttpResponse {
    println!("Someone wants all the users of the forum");
    let mut jsonVec: Vec<User> = vec![];
    let mut cursor: mongodb::Cursor<_> = client.database("userStorage").collection::<User>("users").find(None, None).await.unwrap();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                println!("{:#?}", document);
                jsonVec.push(document);
            }
            Err(e) => {
                eprintln!("There was an error: {}", e.to_string());
            },
        }
    }
    HttpResponse::Ok().json(jsonVec)
}

/*
pub async fn search_uuid(client: Data<Arc<Client>>, user_id: web::Path<User>) -> HttpResponse {
    let uuid_string = user_id.user_id.to_string();
    let filter = doc! { "user_id": uuid_string };
    let find_options = FindOneOptions::builder().build();
    let result = client.database("userStorage").collection::<User>("users").find_one(filter, find_options).await;
*/