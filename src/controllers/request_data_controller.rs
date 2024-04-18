use actix_web::{get, web, HttpResponse};
use uuid::Uuid;
use std::str::FromStr;

#[get("/requestUserid/{id}")]
pub async fn search_uuid(path: web::Path<String>) -> HttpResponse {
    let requested_uuid =  match Uuid::from_str(&path.into_inner()) {
        Ok(val) => val,
        Err(e) => e, // hva skjer her?
    };
    HttpResponse::ImATeapot().body("")
}