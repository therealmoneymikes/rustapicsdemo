use serde::{Deserialize, Serialize};
use cuid::{cuid2, Cuid2Constructor};


pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub role: String
}

pub struct UserResponseObject {
    pub id: String,
    pub email: String,
    pub username: String,
    pub role: String
}
impl User {
    fn new(username: &str, email: &str, password: &str, role: &str) -> Self {
        let id = cuid2();
        User {
            id,
            email: email.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            role: role.to_string()
           

        }
    }
}