use mongodb::{options::ClientOptions, Client};
use dotenv::dotenv;
use std::env;



//Storing Error in the Box type so it's stored on the heap
//dynamic dispatch for all trait objects so that complier knows this value
//will be determine at run type
pub async fn connect_to_db() -> Result<Client, Box<dyn std::error::Error>> {
    //Don't forget to load your env file first!
    dotenv().ok();
    
    let base_client_uri = env::var("MONGO_BASE_URI")?;
    let client_options: ClientOptions = ClientOptions::parse(&base_client_uri).await?; 
    // let client = Client::with_options(client_options)?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}