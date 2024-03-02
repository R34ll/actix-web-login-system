use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub email: String,
    pub password: String,

    first_name: String,
    last_name: String,
    description: String,
}

impl User {
    pub fn new(uuid: String, username: String, email: String, password: String) -> Self {
        Self {
            uuid,
            username,
            email,
            password,
            first_name: String::new(),
            last_name: String::new(),
            description: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}



#[derive(Deserialize, Serialize, Clone)]
pub struct UserSession{
    pub logged: bool,
    pub username: String,
    pub email: String,
    pub first_name:String,
    pub last_name: String,
    pub description:String
}

impl UserSession{
    /// Creates a new [`UserSession`].
    pub fn new(logged:bool, username:String, email:String, first_name:String, last_name:String, description:String)->Self{
        Self{
            logged,
            username,
            email,
            first_name,
            last_name,
            description
        }
    }
}


// pub struct UpdateUser; // Update or exclude user
