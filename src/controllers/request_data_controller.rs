use crate::models::user_model::User;

use actix_web::{
    get, web, HttpResponse,
    web::Data};
use mongodb::{bson::doc, options::FindOneOptions, Client};
use std::sync::Arc;

#[get("/requestUserid/{id}")]
pub async fn search_uuid(client: Data<Arc<Client>>, user_id: web::Path<User>) -> HttpResponse {
    let uuid_string = user_id.user_id.to_string();
    let filter = doc! { "user_id": uuid_string };
    let find_options = FindOneOptions::builder().build();
    let result = client.database("userStorage").collection::<User>("users").find_one(filter, find_options).await;
    HttpResponse::ImATeapot().json(result.unwrap())
}