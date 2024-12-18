mod models;
mod controllers;
mod views;
mod routes;
mod db;
mod services;

use axum::{routing::{put, post, delete, get}, Router};
use controllers::user_controller::{delete_user, get_user};
use std::net::SocketAddr;
use tokio::{self, runtime};
use dotenv::dotenv;




async fn main_api() -> Result<(), Box<dyn std::error::Error>>{
    // let base_url = "http://localhost:3000";
    // //   .route("/login", post(create_user))
    // //   .route("/users/update/:id", put(update_user));
  let app = Router::new()
  .route("/delete", delete(delete_user))
  .route("/users/get/:id", get(get_user));

  let address = SocketAddr::from(([127, 0, 0, 1], 3000));
  println!("Local hostserver is at http://{}", address);


  axum::Server::bind(&address)
  .serve(app.into_make_service()).await.unwrap();

  Ok(())
}

// Remember guys if you want to use async fn main
//Use tokio or async-std to enable it because by default it is not allowed

//Use tokio::main macro to enable the main function to be asynchronus
#[tokio::main]
 async fn main() -> Result<(), Box<dyn std::error::Error>> { 
  println!("Server Started");
  main_api().await?; //If there is an error caught from our Box pointer we can show it here
  Ok(()) 
} 

//Alter
