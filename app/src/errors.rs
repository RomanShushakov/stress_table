use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;


#[derive(Display, From, Debug)]
pub enum MyError
{
    NotFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}
impl std::error::Error for MyError {}


impl ResponseError for MyError
{
    fn error_response(&self) -> HttpResponse
    {
        match *self
        {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::PoolError(ref err) =>
                {
                    HttpResponse::InternalServerError().body(err.to_string())
                },
            MyError::PGError(ref err) =>
                {
                    if err.to_string().contains("users_email_key")
                    {
                        let error_message = "The inputted email is already in use.";
                        return HttpResponse::InternalServerError().body(error_message);
                    }
                    HttpResponse::InternalServerError().finish()
                },
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
