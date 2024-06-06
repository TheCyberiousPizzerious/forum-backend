use crate::models::message_model::{MessageMessage, ErrorMessage, MessageTraits, RegisterSending};
use crate::models::post_models::{self, Post, SpecialPostRelInfoRec, Thread};
use crate::models::user_model::{User, UserId}; 
use crate::models::monitor_models::{
    RegisterLoginLog, LoginLog
};
use crate::utils::utils::{
    bson_now, generate_bson_uuid,b_uuid_decoding
};

use actix_web::{get, web};
use actix_web::{delete, post, put, HttpResponse, web::{Data, Json}, HttpRequest};
use bson::to_bson;
use futures_util::StreamExt;
use mongodb::options::FindOneOptions;
use mongodb::{bson::{oid::ObjectId, Bson, doc}, Client};

use std::os::windows::thread;
use std::sync::Arc;    

#[post("/createThread")]
pub async fn create_thread(client: Data<Arc<Client>>, sendt_data: Json<SpecialPostRelInfoRec>) -> HttpResponse {
    println!("Someone is trying to create a thread");
    println!("userId:        {}", sendt_data.user_id);
    println!("username:     {}", sendt_data.username);
    println!("header:    {}", sendt_data.header.as_ref().unwrap());
    println!("content:    {}", sendt_data.content);
    println!("usertimestamp:     {}", sendt_data.user_timestamp);
    let thread_info: Thread = Thread {
        _id: Some(ObjectId::new()),
        thread_id: Some(UserId::Bson(generate_bson_uuid())),
        user_id: sendt_data.user_id.to_owned(), 
        username: sendt_data.username.to_owned(),
        user_timestamp: sendt_data.user_timestamp.to_owned(),
        board_id: sendt_data.board_id.to_owned(),
        content: sendt_data.content.to_owned(),
        header: sendt_data.header.to_owned().unwrap(),
        timestamp: bson_now(),
    };
    let bson_thread = to_bson(&thread_info).unwrap();
    if let Bson::Document(thread_document) = bson_thread {
        match client.database("postStorage").collection("threads").insert_one(thread_document, None).await {
            Ok(_) => {
                let decoded_uuid = b_uuid_decoding(&thread_info.thread_id.unwrap());
                HttpResponse::Ok().json(MessageMessage::new_from(decoded_uuid.to_string()))
            }, // Needs to send another message
            Err(e) => {
                println!("There was an issue creating the thread, error: {}", e.to_string());
                HttpResponse::InternalServerError().json(ErrorMessage::new_from(e.to_string()))
            },
        }
    } else {
        println!("Document is not valid bson");
        HttpResponse::InternalServerError().json(ErrorMessage::new_from("Error occured internally converting objects between types".to_string()))
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

#[get("/getThreadsBoardId/{board_id}")]
pub async fn get_threads_board_id(client: Data<Arc<Client>>, board_id: web::Path<String>) -> HttpResponse {
    println!("Someone wants all the threads relating to a board id");
    let mut jsonvec: Vec<Thread> = vec![];
    let dboard_id = board_id.into_inner();
    println!("dbourd_id: {}", &dboard_id);
    let filter = doc! { "board_id": dboard_id };
    let mut cursor: mongodb::Cursor<_> = client.database("postStorage").collection::<Thread>("threads").find(filter, None).await.unwrap();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(mut document) => {
                let decoded_thread_id = b_uuid_decoding(&document.thread_id.unwrap());
                document.thread_id = Some(UserId::Uuid(decoded_thread_id));
                println!("board id: {}", &document.board_id);
                jsonvec.push(document);
            }
            Err(e) => {
                eprintln!("There was an error: {}", e.to_string());
            },
        }
    }
    HttpResponse::Ok().json(jsonvec)
}

#[get("/getThreadId/{thread_id}")]
pub async fn get_thread_id(client: Data<Arc<Client>>, thread_id: web::Path<String>) -> HttpResponse {
    println!("Someone wants to get a thread by uuid");
    println!("The Thread: {}", thread_id);
    let filter = doc! { "thread_id": thread_id.to_string() };
    let find_options = FindOneOptions::builder().build();
    let result = client.database("postStorage").collection::<Thread>("threads").find_one(filter, find_options).await;
    match result {
        Ok(val) => {
            match val {
                Some(thread) => {
                    HttpResponse::Ok().json(thread)
                },
                None => HttpResponse::NotFound().json(MessageMessage::new_from(String::from("no user with that username"))),
            }
        },
        Err(e) => {
            println!("We are searching weirdly");
            HttpResponse::NotFound().json(ErrorMessage::new_from(e.to_string()))
        },
    }
}

#[get("/getPostsThreadId/{thread_id}")]
pub async fn get_posts_thread_id(client: Data<Arc<Client>>, thread_id: web::Path<String>) -> HttpResponse {
    println!("Someone wants all the threads relating to a board id");
    let mut jsonvec: Vec<Post> = vec![];
    let dthread_id = thread_id.into_inner();
    println!("dbourd_id: {}", &dthread_id);
    let filter = doc! { "board_id": dthread_id };
    let mut cursor: mongodb::Cursor<_> = client.database("postStorage").collection::<Post>("posts").find(filter, None).await.unwrap();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(mut document) => {
                let decoded_thread_id = b_uuid_decoding(&document.thread_id.unwrap());
                document.thread_id = Some(UserId::Uuid(decoded_thread_id));
                println!("thread id: {}", &document.thread_id.clone().unwrap());
                jsonvec.push(document);
            }
            Err(e) => {
                eprintln!("There was an error: {}", e.to_string());
            },
        }
    }
    HttpResponse::Ok().json(jsonvec)
}

#[post("createPost")]
pub async fn create_post(client: Data<Arc<Client>>, sendt_data: Json<SpecialPostRelInfoRec>) -> HttpResponse {
    println!("userId:        {}", sendt_data.user_id);
    println!("username:     {}", sendt_data.username);
    println!("header:    {}", sendt_data.header.as_ref().unwrap());
    println!("content:    {}", sendt_data.content);
    println!("usertimestamp:     {}", sendt_data.user_timestamp);
    let post_info: Post = Post {
        _id: Some(ObjectId::new()),
        post_id: UserId::Bson(generate_bson_uuid()),
        user_id: sendt_data.user_id.to_owned(), 
        thread_id: sendt_data.thread_id.to_owned(),
        replying_to: sendt_data.replying_to.to_owned(),
        username: sendt_data.username.to_owned(),
        user_timestamp: sendt_data.user_timestamp.to_owned(),
        content: sendt_data.content.to_owned(),
        timestamp: bson_now(),
    };
    let bson_post = to_bson(&post_info).unwrap();
    if let Bson::Document(post_document) = bson_post {
        match client.database("postStorage").collection("post").insert_one(post_document, None).await {
            Ok(_) => {
                let decoded_uuid = b_uuid_decoding(&post_info.post_id);
                HttpResponse::Ok().json(MessageMessage::new_from(decoded_uuid.to_string()))
            }, // Needs to send another message
            Err(e) => {
                println!("There was an issue creating the post, error: {}", e.to_string());
                HttpResponse::InternalServerError().json(ErrorMessage::new_from(e.to_string()))
            },
        }
    } else {
        println!("Document is not valid bson");
        HttpResponse::InternalServerError().json(ErrorMessage::new_from("Error occured internally converting objects between types".to_string()))
    }
}