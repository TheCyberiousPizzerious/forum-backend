mod controllers;
mod models;
mod mongo_repo;
mod utils;

use crate::controllers::user_controller::register;
use crate::mongo_repo::utils::establish_connection;
use crate::utils::utils::{is_api_reachable, grab_info, send_data};

use mongodb::Client;
use actix_web::{App, HttpServer, web::{Data, scope}};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
   let client: Client = establish_connection().await.unwrap();
   let conn_status = match client.list_database_names(None, None).await {
      Ok(_) => "OK".to_string(),
      Err(_) => panic!("FAILED CONNECTING TO DATABASE, ARE YOU SURE YOU HAVE THE RIGHT SERVER IP?"),
   };

   println!("Connection status: {}", conn_status);

   let client: Data<Arc<Client>> = Data::new(Arc::new(client));
   //let user_storage_collection: Data<Arc<Collection<Document>>> = Data::new(Arc::new(get_collection(client, String::from("userStorage"), String::from("users"))));
   println!("we are past storage definition");
   HttpServer::new(move || {
      App::new()
         .app_data(client.clone())
         .service(
            scope("/test")
               .service(is_api_reachable) //ping-server
               .service(grab_info)
               .service(send_data)
         .service(
            scope("/api")
              .service(register)
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