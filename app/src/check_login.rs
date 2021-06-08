use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{http, Error, HttpResponse};
use futures::future::{ok, Either, Ready};

use actix_session::{UserSession};


pub struct CheckLogin;


impl<S, B> Transform<S> for CheckLogin
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckLoginMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;


    fn new_transform(&self, service: S) -> Self::Future
    {
        ok(CheckLoginMiddleware { service })
    }
}


pub struct CheckLoginMiddleware<S>
{
    service: S,
}


impl<S, B> Service for CheckLoginMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;


    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>>
    {
        self.service.poll_ready(cx)
    }


    fn call(&mut self, req: ServiceRequest) -> Self::Future
    {
        let session = req.get_session();

        let auth_status = session.get::<bool>("auth").unwrap();

        let is_logged_in =
            {
                if let Some(status) = auth_status
                {
                    if status
                    {
                        true
                    }
                    else
                    {
                        false
                    }
                }
                else
                {
                    false
                }
            };

        if is_logged_in
        {
            Either::Left(self.service.call(req))
        }
        else
        {
            Either::Right(ok(req.into_response(
                HttpResponse::Found()
                    .header(http::header::LOCATION, "/auth")
                    .finish()
                    .into_body(),
            )))
        }
    }
}
