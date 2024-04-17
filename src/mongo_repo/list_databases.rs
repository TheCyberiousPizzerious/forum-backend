use mongodb::Client;
use std::error::Error;

pub async fn _list_all_databases(client: Client) -> Result<(), Box<dyn Error>> {
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
       println!("- {}", name);
    }
    Ok(())
}