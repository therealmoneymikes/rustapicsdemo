use serde::{Deserialize, Serialize};



pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub role: String
}