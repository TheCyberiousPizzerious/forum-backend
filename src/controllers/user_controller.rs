use crate::models::message_model::{MessageMessage, ErrorMessage, MessageTraits};
use crate::models::user_model::User; 
use crate::models::monitor_models::RegisterLoginLog;
use crate::utils::utils::{to_json, bson_now};
use std::time::Duration;

use actix_web::{delete, get, post, put, HttpResponse, web::{Data, Json}, HttpRequest};
use bson::to_bson;
use mongodb::{bson::{oid::ObjectId, Bson, doc}, Client};
use std::sync::Arc;
use uuid::Uuid;

#[post("/register")]
pub async fn register(req: HttpRequest, client: Data<Arc<Client>>, sendt_data: Json<User>) -> HttpResponse {
    println!("I GOT YOUR REQUEST");
    let ip = req.peer_addr().unwrap();

    let user_data: User = User {
        _id: ObjectId::new(),
        user_id: Uuid::new_v4(),
        admin: false,
        banned: false,
        username: sendt_data.username.to_owned(),
        email: sendt_data.email.to_owned(),
        passwordhash: sendt_data.passwordhash.to_owned(),
        server_timestamp: bson_now(), // Server timestamp
        user_timestamp: sendt_data.user_timestamp.to_owned(),
    };
    match client.database("userStorage").collection::<User>("users").find_one(doc! {"username": &user_data.username}, None).await {
        Ok(result) => {
            match result {
                Some(user) => {
                    if user_data.username == user.username {
                        HttpResponse::Conflict().json(ErrorMessage::new_from("The username is already taken!".to_string()))
                    } else {
                        let log_data = RegisterLoginLog {
                            _id: ObjectId::new(),
                            monitor_id: Uuid::new_v4(),
                            user_id: user_data.user_id,
                            address: ip.to_string(),
                            timestamp: bson_now(),
                        };
                        let bson_user = to_bson(&user_data).unwrap();
                        if let Bson::Document(user_document) = bson_user {
                            match client.database("userStorage").collection("users").insert_one(user_document, None).await {
                                Ok(_) => HttpResponse::Ok().json(ErrorMessage::new_from(user_data.user_id.to_string())),
                                Err(e) => HttpResponse::InternalServerError().json(ErrorMessage::new_from(e.to_string())),
                            }
                        } else {
                            HttpResponse::InternalServerError().json(ErrorMessage::new_from("Error occured internally converting objects between types".to_string()))
                        };
                        
                        let bson_log = to_bson(&log_data).unwrap();
                        if let Bson::Document(log_document) = bson_log {
                            match client.database("monitoringStorage").collection("registerLogs").insert_one(log_document, None).await {
                                Ok(_) => HttpResponse::Ok().json(ErrorMessage::new_from(user_data.user_id.to_string())),
                                Err(e) => HttpResponse::InternalServerError().json(ErrorMessage::new_from(e.to_string())),
                            }
                        } else {
                            HttpResponse::InternalServerError().json(ErrorMessage::new_from("Error occured internally converting objects between types".to_string()))
                        }
                        }
                },
                None => {
                    HttpResponse::InternalServerError().json(ErrorMessage::new_from("There was an error with our mongodb database".to_string()))
                }
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(ErrorMessage::new_from(e.to_string()))
        }
    }

    // every handeling of json to bson to document needs to be moved here

    //client.database("userStorage");
    // Burde kalle på en login funksjon og logge inn for brukeren
}

#[post("/login")]
pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("FINNISH THIS")
}

#[put("/ban/{id}")]
pub async fn ban_user() -> HttpResponse {
    HttpResponse::Ok().body("FINNISH THIS")
}

#[delete("/delete/{id}")]
pub async fn delete_user() -> HttpResponse {
    HttpResponse::Ok().body("FINNISH THIS")
}

#[put("/edit")]
pub async fn edit_user() -> HttpResponse {
    HttpResponse::Ok().body("FINNISH THIS")
}