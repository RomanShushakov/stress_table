use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;
use crypto::sha2::Sha256;
use crypto::digest::Digest;


fn modify_password(password: &str) -> String
{
    let mut updated_password = String::new();
    for (i, char) in password.chars().rev().enumerate()
    {
        if i % 2 == 0
        {
            updated_password += &char.to_uppercase().to_string();
        }
        else
        {
            updated_password += &char.to_string();
        }
    }
    let mut modified_password = Sha256::new();
    modified_password.input_str(updated_password.as_str());
    modified_password.result_str()
}


#[derive(Deserialize)]
pub struct UserDataFromClientForRegistration
{
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}


#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
pub struct User
{
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub is_active: bool,
    pub is_superuser: bool,
    pub username: String,
}


impl User
{
    pub fn create(user_data_from_client_for_registration: UserDataFromClientForRegistration) -> Self
    {
        let id  = Uuid::new_v4();
        let first_name = user_data_from_client_for_registration.first_name;
        let last_name = user_data_from_client_for_registration.last_name;
        let username = (first_name.chars().nth(0).unwrap().to_string() +
            &last_name.chars().nth(0).unwrap().to_string()).to_uppercase();
        let email = user_data_from_client_for_registration.email;
        let password = modify_password(&user_data_from_client_for_registration.password);
        let is_active = true;
        let is_superuser = false;
        User { id, first_name, last_name, email, password, is_active, is_superuser, username }
    }
}


#[derive(Deserialize)]
pub struct UserDataFromClientForLogin
{
    pub email: String,
    pub password: String,
}


#[derive(Deserialize)]
pub struct UserDataForLogin
{
    pub email: String,
    pub password: String,
}


impl UserDataForLogin
{
    pub fn create(user_data_from_client_for_login: UserDataFromClientForLogin) -> Self
    {
        let email = user_data_from_client_for_login.email;
        let password = modify_password(&user_data_from_client_for_login.password);
        UserDataForLogin { email, password }
    }
}


#[derive(Serialize)]
pub struct AuthorizedUserInfo
{
    pub username: String,
}
