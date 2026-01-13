use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize)]
pub enum ApiResponse<T> {
    Success { data : T, message: String},
    Error { message: String }
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String
}

/* 
    This makes the userRespnse struct copy just the id, name, email
    and remove senstive information like 'password' in this case
*/
impl From<super::models::User> for UserResponse {
    fn from (user: super::models::User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct PublicUser{
    pub id: i32,
    pub name: String,
    pub email: String
}

impl From<super::models::User> for PublicUser {
    fn from (user: super::models::User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email
        }
    }
}

