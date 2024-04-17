mod controllers;
mod models;
mod mongo_repo;
mod utils;

use crate::controllers::user_controller::register_user;
use crate::mongo_repo::mongo_connection::establish_connection;
use crate::utils::utils::{is_api_reachable, get_collection};

use mongodb::{Client, Collection, bson::Document};
use actix_web::{App, HttpServer, web::{Data, scope}};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
   let client: Client = establish_connection().await.unwrap();
   let conn_status = match client.list_database_names(None, None).await {
      Ok(_) => "OK".to_string(),
      Err(_) => "FAILED".to_string(),
   };

   println!("Connection status: {:?}", conn_status);

   //let client: Data<Arc<Client>> = Data::new(Arc::new(client));
   let user_storage_collection: Data<Arc<Collection<Document>>> = Data::new(Arc::new(get_collection(client, String::from("userStorage"), String::from("users"))));
   println!("we are past storage definition");
   HttpServer::new(move || {
      App::new()
         .app_data(user_storage_collection.clone())
         .service(
            scope("/test")
               .service(is_api_reachable)
         .service(
            scope("/api")
              .service(register_user)
         )
            // api
               //requestData
               //requestLogs
               //userHandeler
               //utilityHandeler
         )
   })
   .bind("127.0.0.1:7175")?
   .run()
   .await
}