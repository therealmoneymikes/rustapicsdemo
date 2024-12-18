
use mongodb::{bson, Collection};
use crate::models::user::{User, hash_password};
use std::error::Error;

use crate::db::db::connect_to_db;

pub enum Role {
    Guest,
    User,
    Admin
}



pub async fn create_user_service(payload: User) -> Result<User, Box<dyn Error>> {
    let db_client = connect_to_db().await?;//Call our connect to db function
    let collection: Collection<bson::Document> = db_client.database("practice").collection("users");

    //Checking if user already exists in the db by their email - THIS IS THE MOST USER FRIENDLY OPTION!
    // &payload.email - I've passed a reference of payload object because we only need to read it here 
    let validation_filter = bson::doc! {"email": &payload.email}; 
    
    //Here we checking collection.find_one's result is equal to our User document (if it exists)
    if let Some(document) = collection.find_one(validation_filter).await? {
        //Checking to see if we already have a user with that email in the db
        let user: User = bson::from_document(document)?;
        println!("User found: {:?}", user);
        return Ok(user);
    }

    //If user doesn't exist we can now create a new user w
    
    let new_user = User {
        id: cuid::cuid2().clone(),
        email: payload.email.clone(),
        username: payload.username.clone(),
        password: hash_password(&payload.password, &[10])?,
        role: "User".to_string()

    };

    // let new_user_document =
    let new_user_doc = bson::to_document(&new_user)?;//Here we have serialize the new_user object into a 
    //bson::Document which is binary json doc we get 
    let res = collection.insert_one(new_user_doc).await?;
    println!("New user created with ID: {:?}", res.inserted_id); //Adding Object Id 

          
    Ok(new_user)
}


//If the results is multiple values use .to_vec to convert it an vector array
//For single values use .next() to get value from the cursor

//if let Some(user) = existing_user checks if existing_user is Some(user) (i.e., a user was found in the database). 
//If it is, it extracts the user and executes the block inside the if let.

//If existing_user is None (i.e., no user was found), it skips the if let block and proceeds with the code after else, 
//which creates a new user and inserts it into the database.


