use std::sync::Arc;

use axum::{extract::{Json, Path}, http::header::DATE, response::Json as AxumJson};
use mongodb::Collection;
use crate::models::user::{User, UserResponseObject};
use crate::views::user_view::{success_response_format, user_response_format};

//Atomically Reference Counted DB Alias


pub async fn create_user(Json(payload): Json<User>) -> AxumJson<String> {


    

    let user = User {
        id: payload.id,
        username: payload.username,
        email: payload.email,
        password: payload.password,
        role: payload.role
    };

    AxumJson(success_response_format("User created succesfully", &user))
    //Returned formatted success response for the 201 response
}

pub async fn get_user(Path(id): Path<u32>) -> AxumJson<String> {

    let user = User {
        id: id.to_string(),
        username: format!("User{}", id.to_string()),
        email: format!("user{}@demp.com", id.to_string()),
        password: "12345678".to_string(),
        role: "User".to_string()
    };
    AxumJson(user_response_format(&user))
}

pub async fn update_user(Path(id): Path<u32>, Json(payload): Json<User>) -> AxumJson<UserResponseObject> {

    let updated_user = User {
        id: payload.id.clone(),
        email: payload.email.clone(),
        username: payload.username.clone(),
        password: payload.password.clone(),
        role: payload.role.clone()
   
     };

     let user_response = UserResponseObject {
        id: updated_user.id,
        username: updated_user.username, //Note to Self, username and email are optional for now
        email: updated_user.email,
        role: updated_user.role
     };

     AxumJson(user_response)
    }

pub async fn delete_user(Path(id): Path<u32>) -> AxumJson<String> {
    AxumJson(format!("{{ \"message\": \"User with id {} deleted successfully at {}\" }}", id, DATE.as_str()))
}