use crate::models::user::User;


//Return id, username and email,
pub fn user_response_format(user: &User) -> String {
    format!("{{\"id\": {}, \"username\": \"{}\", \"email\": \"{}\" }}", user.id, user.username, user.email)
}

pub fn success_response_format(message: &str, user: &User) -> String {
    format!("{{ \"message\": \"{}\", \"name\": {{ \"id\": {}, \"name\": \"{}\", \"email\": \"{}\" }} }}",
        message, user.id, user.username, user.email)
}

