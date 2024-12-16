use mongodb::{options::ClientOptions, Client, Collection};


use crate::models::user::User;
use std::error::Error;
use bson::doc;
use crate::db::db::connect_to_db;






async fn create_user_service(payload: User) -> Result<User, Box<dyn Error>> {
    //Connect to mongo db from 
    let base_client_uri = "mongodb://localhost:27017";
    let client_options: ClientOptions = ClientOptions::parse(base_client_uri).await?; //
    let client = Client::with_options(client_options)?;
    

    let main_database = client.database("practice"); //You can please your own db name here
    let collection: Collection<User> = main_database.collection("users");

    //Checking if user already exists in the db by their email

    let validation = doc! {"email": &payload.email}; 
    //I've passed a reference of payload object because we only need to read it here 
    let existing_user = collection.find(validation).await?;


    //If the results is multiple values use .to_vec to convert it an vector array
    //For single values use .next() to get value from the cursor

    //if let Some(user) = existing_user checks if existing_user is Some(user) (i.e., a user was found in the database). 
    //If it is, it extracts the user and executes the block inside the if let.

    //If existing_user is None (i.e., no user was found), it skips the if let block and proceeds with the code after else, 
    //which creates a new user and inserts it into the database.
    if let Some(user) = existing_user {
        Ok(user)
    } else {
        let new_user = User {
            id: cuid::cuid2(),
            email: payload.email,
            username: payload.username,
            password: payload.password,
            role: "User".to_string()

        };

        collection.insert_one(new_user, None).await?;

        Ok(new_user)
    }


}