use actix_web::{cookie::time::Date, get, rt::System, web::Json, HttpResponse, Responder};
use mongodb::{Collection, Client, bson::{Bson, to_bson, Document, DateTime}};
use serde::{Deserialize, Serialize};
use serde_json::{to_string, Error};
use std::time::SystemTime;

/// Returns the selected Collection in the selected Database
/// ```no_run
/// let collection = get_collection(client_instance, database, collection);
/// collection.insert_one("blahblahblah");
/// ```
pub fn get_collection(client: Client, database: String, collection: String) -> Collection<Document> { 
    let db = client.database(&database);
    db.collection(&collection)
}

/// Test the availability of the server
#[get("/ping_server")]
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
pub fn to_json<T: Serialize>(data: T) -> Result<String, Error> {
    match to_string(&data) {
        Ok(val) => Ok(val),
        Err(e) => Err(e),
    }
}

pub fn bson_now() -> DateTime {
    let now: SystemTime = SystemTime::now();
    let bson_now: DateTime = DateTime::from(now);
    bson_now
}

// Wip
//pub fn from_json<T: Deserialize>(json: T) -> Result<T, Error> {
//    Ok(T)
//}