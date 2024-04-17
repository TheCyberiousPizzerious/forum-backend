use dotenv::dotenv;
use mongodb::{Client, error::Error, Collection, bson::Document};
use std::env;
//use url_encoder;

pub async fn establish_connection() -> Result<Client, Error> {
    println!("ESTABLISHING CONNECTION TO MONGODB SERVER");
    dotenv().ok();
    let mongouri_env = env::var("MONGOURI_VM").unwrap();
    println!("Environment variable: {}", mongouri_env);
    match Client::with_uri_str(mongouri_env).await {
        Ok(client) => {
            println!("Client: {:?}", client);
            Ok(client)
        },
        Err(e) => Err(Error::from(e)),
    }
}

/// Returns the selected Collection in the selected Database
/// ```no_run
/// let collection = get_collection(client_instance, database, collection);
/// collection.insert_one("blahblahblah");
/// ```
pub fn get_collection(client: Client, database: String, collection: String) -> Collection<Document> { 
    let db = client.database(&database);
    db.collection(&collection)
}