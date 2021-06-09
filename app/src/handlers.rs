use actix_web::{web, HttpResponse, Result, http};
use actix_session::Session;
use deadpool_postgres::{Client, Pool};

use crate::db;
use crate::errors::MyError;
use crate::models::{UserDataFromClientForRegistration, UserDataFromClientForLogin, Messages};


pub async fn register(user_data_from_client_for_registration: web::Json<UserDataFromClientForRegistration>,
    db_pool: web::Data<Pool>) -> Result<HttpResponse>
{
    let user_data_from_client_for_registration: UserDataFromClientForRegistration =
        user_data_from_client_for_registration.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let _ = db::add_user(&client, user_data_from_client_for_registration).await?;

    Ok(HttpResponse::Ok().body("You was successfully registered!"))
}


pub async fn login(user_data_from_client_for_login: web::Json<UserDataFromClientForLogin>,
    db_pool: web::Data<Pool>, session: Session) -> Result<HttpResponse>
{
    let user_data_from_client_for_login: UserDataFromClientForLogin =
        user_data_from_client_for_login.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let (user_id, username) =
        db::get_user(&client, user_data_from_client_for_login).await?;

    session.set("username", username)?;
    session.set("user_id", user_id)?;
    session.set("auth", true)?;
    session.renew();

    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish()
        .into_body())
}


pub async fn logout(session: Session) -> Result<HttpResponse>
{
    let auth_status: Option<bool> = session.get("auth")?;
    if let Some(_status) = auth_status
    {
        session.purge();
        Ok(HttpResponse::Found()
            .header(http::header::LOCATION, "/auth")
            .finish()
            .into_body())
    }
    else
    {
        Ok("Could not log out anonymous user".into())
    }
}


pub async fn update_cache(session: Session, message: String) -> Result<HttpResponse>
{
    let messages: Vec<String> = session
        .get::<Vec<String>>("messages")
        .unwrap_or(Some(Vec::new()))
        .map_or(vec![message.clone()], |mut inner| { inner.push(message); inner });
    session.set("messages", messages.clone())?;
    Ok(HttpResponse::Ok().finish())
}


pub async fn load_cache(session: Session) -> Result<HttpResponse>
{
    let messages: Vec<String> = session.get::<Vec<String>>("messages")
        .or(Err(MyError::GetMessagesError))?
        .unwrap_or(Vec::new());
    Ok(HttpResponse::Ok().json(Messages { messages }))
}
