use crate::models::{message_model::{ErrorMessage, SendWhatever, WhatevTraits}, user_model::{User, UserId}};

use crate::models::message_model::{MessageMessage, MessageTraits};

use actix_web::http::header::Allow;
use actix_web::{
    get, web::{self, Data}, HttpResponse};
use base64::{Engine as _, alphabet, engine::general_purpose::STANDARD};
use bson::spec::BinarySubtype;
use bson::Binary;
use bson::from_document;
use mongodb::{bson::doc, options::{FindOneOptions, UpdateOptions}, Client};
use futures_util::StreamExt;
use std::sync::Arc;
use uuid::Uuid;

#[get("/requestUserId/{id}")] // This one is wrong and needs to check if the user is real
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

#[get("/getUserUsername/{username}")]
pub async fn get_user_username(client: Data<Arc<Client>>, username: web::Path<String>) -> HttpResponse {
    println!("Someone wants to know if a user is admin");
    println!("The username: {}", username);
    let filter = doc! { "username": username.to_string() };
    let find_options = FindOneOptions::builder().build();
    let result = client.database("userStorage").collection::<User>("users").find_one(filter, find_options).await;
    match result {
        Ok(val) => {
            match val {
                Some(user) => {
                    HttpResponse::Ok().json(user)
                },
                None => HttpResponse::NotFound().json(MessageMessage::new_from(String::from("no user with that username"))),
            }
        },
        Err(e) => HttpResponse::NotFound().json(ErrorMessage::new_from(e.to_string()))
    }
}

#[get("/getAllUsers")]
pub async fn get_all_users(client: Data<Arc<Client>>) -> HttpResponse {
    println!("Someone wants all the users of the forum");
    let mut jsonvec: Vec<User> = vec![];
    let mut cursor: mongodb::Cursor<_> = client.database("userStorage").collection::<User>("users").find(None, None).await.unwrap();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(mut document) => {
                if let Some(user_id) = &document.user_id {
                    match user_id {
                        UserId::Bson(bson::Bson::Binary(Binary { subtype, bytes })) => {
                            if *subtype == BinarySubtype::Uuid {
                                match Uuid::from_slice(bytes) {
                                    Ok(uuid) => {
                                        document.user_id = Some(UserId::Uuid(uuid));
                                    },
                                    Err(e) => {
                                        eprintln!("Failed to decode binary uuid, error: {}", e);
                                    },
                                }
                            } else {
                                println!("The subtype is not binary");
                            }
                        },
                        _ => {
                            eprintln!("user_id is not Bson::Binary");
                        },
                    }
                } else {
                    eprintln!("Did not find a field user_id in document");
                }
                jsonvec.push(document);
            }
            Err(e) => {
                eprintln!("There was an error: {}", e.to_string());
            },
        }
    }
    HttpResponse::Ok().json(jsonvec)
}

#[get("/isAdminUsername/{username}")]
pub async fn is_admin_username(client: Data<Arc<Client>>, username: web::Path<String>) -> HttpResponse {
    println!("Someone wants to know if a user is admin");
    println!("The username: {}", username);
    let filter = doc! { "username": username.to_string() };
    let find_options = FindOneOptions::builder().build();
    let result = client.database("userStorage").collection::<User>("users").find_one(filter, find_options).await;
    match result {
        Ok(val) => {
            match val {
                Some(user) => {
                    if user.admin == Some(true) {
                        HttpResponse::Ok().json(MessageMessage::new_from("The user is admin".to_string()))
                    } else {
                        HttpResponse::Forbidden().json(MessageMessage::new_from("The user is not admin".to_string()))
                    }
                },
                None => HttpResponse::NotFound().json(MessageMessage::new_from(String::from("no user with that username"))),
            }
        },
        Err(e) => HttpResponse::NotFound().json(ErrorMessage::new_from(e.to_string()))
    }
}

#[get("/makeAdminUsername/{username}/{makeOrRemoveAdmin}")]
pub async fn make_admin_username(client: Data<Arc<Client>>, path: web::Path<(String, bool)>) -> HttpResponse {
    let (username, mut adminbool) = path.into_inner();
    if adminbool != true || false {
        adminbool = false 
    }
    println!("Someone wants to know if a user is admin");
    println!("The username: {}", username);
    let filter = doc! { "username": &username };
    let update = doc! { "$set": { "admin": &adminbool} };
    let result = client.database("userStorage").collection::<User>("users").update_one(filter, update, None).await;
    match result {
        Ok(val) => {
            println!("update result: {:?}", val);
            HttpResponse::Ok().json(MessageMessage::new_from(adminbool.to_string()))
        },
        Err(e) => HttpResponse::NotFound().json(ErrorMessage::new_from(e.to_string()))
    }
}

/*
pub async fn search_uuid(client: Data<Arc<Client>>, user_id: web::Path<User>) -> HttpResponse {
    let uuid_string = user_id.user_id.to_string();
    let filter = doc! { "user_id": uuid_string };
    let find_options = FindOneOptions::builder().build();
    let result = client.database("userStorage").collection::<User>("users").find_one(filter, find_options).await;
*/