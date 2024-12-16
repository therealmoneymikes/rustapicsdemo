use mongodb::{options::ClientOptions, Client};





//Storing Error in the Box type so it's stored on the heap
//dynamic dispatch for all trait objects so that complier knows this value
//will be determine at run type
pub async fn connect_to_db() -> Result<Client, Box<dyn std::error::Error>> {
    let base_client_uri = "mongodb://localhost:27017";
    let client_options: ClientOptions = ClientOptions::parse(base_client_uri).await?; //
    let client = Client::with_options(client_options)?;
    Ok(client)
}