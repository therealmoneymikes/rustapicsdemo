use std::borrow::Borrow;

use bcrypt::BcryptError;
use bson::Regex;
use serde::{Deserialize, Serialize};
use cuid::{cuid2, Cuid2Constructor};


//I've made this reusable but you can set your salt a default value if
pub fn hash_password(password: &str, salt: &[u8]) -> Result<String, BcryptError> {
    let hashed_password = bcrypt::hash_with_salt(password, bcrypt::DEFAULT_COST, salt)?;
    Ok(hashed_password.format_for_version(bcrypt::Version::TwoB).to_string())
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub role: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResponseObject {
    pub id: String,
    pub email: String,
    pub username: String,
    pub role: String
}
impl User {
    // Validators

    pub fn validate_username(username: &str) -> bool {
        !username.is_empty() && username.len() >=2 
        //Here I am checking that username is not empty i.e - ""
        //I'm also checking that username.len is at least characters modify it as you wish
    }

    pub fn validate_email(email: &str) -> bool {
        email.contains("@") && email.contains("[a-zA-Z0-9+@]")
    }

    pub fn validate_role(role: &str, expected_roles: &[&str]) -> bool {
        expected_roles.contains(&role) //The simplest validate is to check if the expected roles array contain that role
        //e.g expected_roles = ["user", "admin"] etc
    }

    // Password Verifications
    //Based on the Object expert prinicple we can provide a verify_password method here 
    pub fn verify_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap_or(false) //If the password do not match this will be false
    }
    pub fn new(username: &str, email: &str, password: &str, role: &str) -> Self {
        let id = cuid2();
        User {
            
            id,
            email: email.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            role: role.to_string()
           

        }
    }

    pub fn new_with_hashed_password(
        username: &str,
        email: &str,
        password: &str,
        role: &str,
        salt:&[u8]
    ) -> Self {
        let hashed_password = hash_password(password, salt).unwrap_or_else(|_|{
            panic!("Failed to hash password")
        });
        
        User {
            id: cuid2(),
            email: email.to_string(),
            username: username.to_string(),
            password: hashed_password,
            role: role.to_string()


    }
}
}
impl From<User> for UserResponseObject {
    fn from(user: User) -> Self {
        UserResponseObject { id: user.id, email: user.email, username: user.username, role: user.role }
    }
}

