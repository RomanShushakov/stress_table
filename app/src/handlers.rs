use actix_web::{web, Error, HttpResponse, Result, http};
use actix_session::Session;
use deadpool_postgres::{Client, Pool};

use crate::db;
use crate::errors::MyError;
use crate::models::{UserDataFromClientForRegistration, UserDataFromClientForLogin};


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
