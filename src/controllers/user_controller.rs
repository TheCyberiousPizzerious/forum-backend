use crate::models::message_model::{MessageMessage, ErrorMessage, MessageTraits, RegisterSending};
use crate::models::user_model::{User, UserId}; 
use crate::models::monitor_models::{
    RegisterLoginLog, LoginLog
};
use crate::utils::utils::{
    b_uuid_decoding, bson_now, generate_bson_uuid
};

use actix_web::{delete, post, put, HttpResponse, web::{Data, Json}, HttpRequest};
use bson::to_bson;
use mongodb::{bson::{oid::ObjectId, Bson, doc}, Client};

use std::sync::Arc;                

#[post("/register")]
pub async fn register(req: HttpRequest, client: Data<Arc<Client>>, sendt_data: Json<User>) -> HttpResponse {
    println!("Someone wants to register a user");
    let ip = req.peer_addr().unwrap();

    println!("username:     {}", sendt_data.username);
    println!("email:        {}", sendt_data.email.as_ref().unwrap());
    println!("password:     {}", sendt_data.passwordhash);
    println!("timestamp:    {}", sendt_data.user_timestamp.as_ref().unwrap());

    let user_data: User = User {
        _id: Some(ObjectId::new()),
        user_id: Some(UserId::Bson(generate_bson_uuid())),
        admin: Some(false),
        banned: Some(false),
        username: sendt_data.username.to_owned(),
        email: sendt_data.email.to_owned(),
        passwordhash: sendt_data.passwordhash.to_owned(), //Needs to be change to password something and the passwords need to be hashed earlier in the file
        server_timestamp: Some(bson_now()), // Server timestamp
        user_timestamp: sendt_data.user_timestamp.to_owned(),
    };
    println!("id: {}", user_data._id.unwrap());
    match client.database("userStorage").collection::<User>("users").find_one(doc! {"username": &user_data.username}, None).await {
        Ok(result) => {
            println!("result from finding: {:?}", result);
            match result {
                Some(v) => { // This should just be the username is already taken
                    println!("User used already used username: {:?}", v);
                    HttpResponse::Conflict().json(ErrorMessage::new_from("The username is already taken!".to_string()))
                },
                None => { // Here is what should happen when the username can be chosen
                    let log_data = RegisterLoginLog {
                        _id: ObjectId::new(),
                        monitor_id: generate_bson_uuid(),
                        user_id: user_data.user_id.clone().unwrap(),
                        address: ip.to_string(),
                        timestamp: bson_now(),
                    };
                    let bson_user = to_bson(&user_data).unwrap();
                    if let Bson::Document(user_document) = bson_user {
                        match client.database("userStorage").collection("users").insert_one(user_document, None).await {
                            Ok(_) => {
                                println!("User created, id: {}", user_data.user_id.clone().unwrap().to_string());
                                let bson_log = to_bson(&log_data).unwrap();
                                if let Bson::Document(log_document) = bson_log {
                                    match client.database("monitoringStorage").collection("registerLogs").insert_one(log_document, None).await {
                                        Ok(_) => {
                                            println!("Register-log og created, id: {}", &log_data.monitor_id);
                                        },
                                        Err(e) => {
                                            println!("There was an error creating the register log, error: {}", e.to_string());
                                        },
                                    }
                                } else {
                                    println!("Something is wrong with the bson data not making it a valid document");
                                    // There needs to be code here where you 
                                };
                                let decoded_id = b_uuid_decoding(&user_data.user_id.unwrap());
                                let sendingdata = RegisterSending {
                                    username: user_data.username,
                                    admin: user_data.admin.unwrap(),
                                    user_id: decoded_id,
                                };
                                HttpResponse::Ok().json(sendingdata)
                            }, // Needs to send another message
                            Err(e) => {
                                println!("There was an issue creating the user, error: {}", e.to_string());
                                HttpResponse::InternalServerError().json(ErrorMessage::new_from(e.to_string()))
                            },
                        }
                    } else {
                        println!("Document is not valid bson");
                        HttpResponse::InternalServerError().json(ErrorMessage::new_from("Error occured internally converting objects between types".to_string()))
                    }
                }
            }
        },
        Err(e) => {
            println!("There was an issue searching the database if the username is already taken");
            HttpResponse::InternalServerError().json(ErrorMessage::new_from(e.to_string()))
        }
    }

    // every handeling of json to bson to document needs to be moved here

    //client.database("userStorage");
    // Burde kalle på en login funksjon og logge inn for brukeren
}

#[post("/login")]
pub async fn login(req: HttpRequest, client: Data<Arc<Client>>, sendt_data: Json<User>) -> HttpResponse {
    println!("Someone is logging in!");
    let mut log_data: LoginLog = LoginLog {
        _id: ObjectId::new(),
        monitor_id: generate_bson_uuid(),
        user_id: UserId::Bson(Bson::Null), // neds to be changed later after the blahblahblah
        address: req.connection_info().peer_addr().unwrap().to_string(),
        is_succesfull: false,
        timestamp: bson_now(),
    };
    match client.database("userStorage").collection::<User>("users").find_one(doc! {"username": &sendt_data.username}, None).await {
        Ok(Some(user_document)) => {
            log_data.user_id = user_document.user_id.clone().unwrap();
            match client.database("userStorage").collection::<User>("users").find_one(doc! {"username": &sendt_data.username, "passwordhash": &sendt_data.passwordhash}, None).await {
                Ok(Some(_)) => {
                    println!("User logged in");
                    log_data.is_succesfull = true;
                    let bson_log = to_bson(&log_data).unwrap();
                    if let Bson::Document(log_document) = bson_log {
                        match client.database("monitoringStorage").collection("registerLogs").insert_one(log_document, None).await {
                            Ok(_) => {
                                println!("login-log og created, id: {}", &log_data.monitor_id);
                            },
                            Err(e) => {
                                println!("There was an error creating the login log, error: {}", e.to_string());
                            },
                        }
                    } else {
                        println!("Something is wrong with the bson data not making it a valid document");
                        // There needs to be code here where you 
                    };
                    let decoded_uuid = b_uuid_decoding(&user_document.user_id.unwrap());
                    let sendingdata = RegisterSending {
                        username: user_document.username,
                        admin: user_document.admin.unwrap(),
                        user_id: decoded_uuid,
                    };
                    HttpResponse::Ok().json(sendingdata)
                },
                Ok(None) => {
                    println!("There is no user with this username or password");
                    log_data.is_succesfull = false;
                    let bson_log = to_bson(&log_data).unwrap();
                    if let Bson::Document(log_document) = bson_log {
                        match client.database("monitoringStorage").collection("registerLogs").insert_one(log_document, None).await {
                            Ok(_) => {
                                println!("login-log og created, id: {}", &log_data.monitor_id);
                            },
                            Err(e) => {
                                println!("There was an error creating the login log, error: {}", e.to_string());
                            },
                        }
                    } else {
                        println!("Something is wrong with the bson data not making it a valid document");
                        // There needs to be code here where you 
                    };
                    HttpResponse::Unauthorized().json(MessageMessage::new_from("Wrong username or password".to_string()))
                }
                Err(e) => {
                    println!("login failed: {}", e.to_string());
                    HttpResponse::InternalServerError().json(ErrorMessage::new_from("There was an error whilts doing a mongodb query".to_string()))
                }
            }
        },
        Ok(None) => {
            println!("There is no user with this username");
            log_data.is_succesfull = false;
                    let bson_log = to_bson(&log_data).unwrap();
                    if let Bson::Document(log_document) = bson_log {
                        match client.database("monitoringStorage").collection("registerLogs").insert_one(log_document, None).await {
                            Ok(_) => {
                                println!("login-log og created, id: {}", &log_data.monitor_id);
                            },
                            Err(e) => {
                                println!("There was an error creating the login log, error: {}", e.to_string());
                            },
                        }
                    } else {
                        println!("Something is wrong with the bson data not making it a valid document");
                        // There needs to be code here where you 
                    };
            HttpResponse::NotFound().json(MessageMessage::new_from("No user with this username".to_string()))
        }
        Err(e) => { // There is no user with this username
            println!("Is this an actial error or does this appear when there is no user with the username: {}", e.to_string());
            HttpResponse::InternalServerError().json(ErrorMessage::new_from("There is no user with this username!".to_string()))
        }
    }
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