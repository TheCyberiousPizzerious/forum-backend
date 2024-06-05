mod controllers;
mod models;
mod mongo_repo;
mod utils;

use crate::controllers::{
   post_controller::{
      create_thread, get_threads_board_id
   },
   request_data_controller::{
      get_all_users, is_admin_username, make_admin_username, search_uuid
   }, user_controller::{
      login, register
   }
};
use crate::mongo_repo::utils::establish_connection;
use crate::utils::utils::{is_api_reachable, grab_info, send_data};

use mongodb::Client;
use actix_cors::Cors;
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
      let cors = Cors::default()
      .allow_any_origin()
      .allow_any_method()
      .allow_any_header()
      .expose_any_header()
      .max_age(3600);

      App::new()
         .wrap(cors)
         .app_data(client.clone())
         .service(
            scope("/test")
               .service(is_api_reachable) //ping-server
               .service(grab_info)
               .service(send_data)
            )
         .service(
            scope("/api")
              .service(is_api_reachable)
              .service(register)
              .service(search_uuid) // Looks for a user based on uuid, name is requestUserid
              .service(get_all_users)
              .service(is_admin_username)
              .service(make_admin_username)
              .service(login)
              .service(create_thread)
              .service(get_threads_board_id)
         )
            // api
               //requestData
               //requestLogs
               //userHandeler
               //utilityHandeler
   })
   .bind("0.0.0.0:7175")?
   .run()
   .await
}