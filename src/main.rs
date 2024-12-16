mod models;
mod controllers;
mod views;
mod routes;
mod db;
mod services;

use axum::{routing::{put, post, delete, get}, Router};
use controllers::user_controller::{delete_user, get_user};
use std::net::SocketAddr;



async fn main_api(){
    
    //   .route("/login", post(create_user))
    //   .route("/users/update/:id", put(update_user));
  let app = Router::new()
  .route("/delete", delete(delete_user))
  .route("/users/get/:id", get(get_user));

  let address = SocketAddr::from(([127, 0, 0, 1], 3000));
  println!("Local hostserver is at http://{}", address);
  axum::Server::bind(&address)
  .serve(app.into_make_service()).await.unwrap();
}

 fn main(){
let base_url = "http://localhost:3000";


} 
