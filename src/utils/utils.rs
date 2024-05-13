use actix_web::{get, post, web::Json, HttpRequest, HttpResponse, Responder};
use mongodb::bson::{Bson, DateTime};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use uuid::Uuid;

use std::time::SystemTime;

/// Test the availability of the server
#[get("/ping-server")]
pub async fn is_api_reachable() -> impl Responder {
 HttpResponse::Ok().json("recieved your call!")
}

/// Makes general types with the Serialize trait into json objects
/// ```no_run
/// struct Gamer {
///     name: String,
/// }
/// 
/// let instance = Gamer {
///     name: "xx_epigamer_xx".to_string()
/// };
/// 
/// println!("result: {}", to_json(instance));
/// // result {"name":"xx_epigamer_xx"}
/// ```
pub fn _to_json<T: Serialize>(data: T) -> Result<String, serde_json::Error> {
    match to_string(&data) {
        Ok(val) => Ok(val),
        Err(e) => Err(e),
    }
}

/// Returns a mongodb bson DateTime instance that is set to the current time
/// keep in mind that this gives milli second precision
/// ```no_run
/// let now = bson_now();
/// ```
pub fn bson_now() -> DateTime {
    let now: SystemTime = SystemTime::now();
    DateTime::from(now)
}

// Wip
//pub fn from_json<T: Deserialize>(json: T) -> Result<T, Error> {
//    Ok(T)
//}

/// # Get all user data
/// retuns a lot of information from the different headers
#[get("/grab-info")]
pub async fn grab_info(info: HttpRequest) -> HttpResponse {
    println!("head:         {:?}", info.head());
    println!("uri:          {:?}", info.uri());
    println!("method:       {:?}", info.method());
    println!("version:      {:?}", info.version());
    println!("headers:      {:?}", info.headers());
    println!("path:         {:?}", info.path());
    println!("query_str:    {:?}", info.query_string());
    println!("match info:   {:?}", info.match_info());
    println!("match pattrn: {:?}", info.match_pattern());
    println!("match name:   {:?}", info.match_name());
    //println!("conn_data:    {:?}", info.conn_data());
    println!("conn info:    {:?}", info.connection_info());
    if let Some(ip_string) = info.connection_info().peer_addr() {
        println!("peer address: {ip_string}");
    };
    //connection_info
    //realip_remote_addr
    //peer_addr
    //host
    //scheme
    HttpResponse::Ok().body("ok!")
}

#[derive(Deserialize, Debug)]
struct Data {
    data: String,
}

#[post("/send-data")]
pub async fn send_data(thing: Json<Data>, req: HttpRequest) -> HttpResponse {
    println!("call to {}", req.uri());
    println!("got some data: {}", thing.data);
    HttpResponse::Ok().body("ok!")
}

pub fn generate_bson_uuid() -> Bson {
    let uuid = Uuid::new_v4();
    Bson::Binary(mongodb::bson::Binary {
        subtype: mongodb::bson::spec::BinarySubtype::Uuid,
        bytes: uuid.as_bytes().to_vec(),
    })
}