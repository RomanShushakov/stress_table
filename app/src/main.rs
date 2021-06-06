use actix_web::{HttpServer, App, middleware, Result, HttpResponse, HttpRequest, http};
use actix_web::web::{get, post, resource, scope, service, Json};
use actix_files::{Files, NamedFile};
use actix_redis::RedisSession;
use actix_session::Session;
use std::path::PathBuf;
use rand::Rng;

mod check_login;
use check_login::CheckLogin;


async fn index(_req: HttpRequest) -> Result<NamedFile>
{
    let index_path: PathBuf = "./web_layout/index.html".parse().unwrap();
    Ok(NamedFile::open(index_path)?)
}


async fn files(req: HttpRequest) -> Result<NamedFile>
{
    let mut path = PathBuf::new();
    let prefix_path = "./web_layout";
    let requested_path: PathBuf = req.match_info().query("filepath").parse().unwrap();
    path.push(prefix_path);
    path.push(requested_path);
    Ok(NamedFile::open(path)?)
}


async fn auth(_req: HttpRequest) -> Result<NamedFile>
{
    let auth_path: PathBuf = "./web_layout/auth.html".parse().unwrap();
    Ok(NamedFile::open(auth_path)?)
}


async fn login(session: Session) -> Result<HttpResponse>
{
    // let id = user_id.into_inner().user_id;
    // session.set("user_id", &id)?;
    session.set("auth", true)?;
    session.renew();

    // let counter: Vec<i32> = session
    //     .get::<Vec<i32>>("counter")
    //     .unwrap_or(Some(vec![0]))
    //     .unwrap_or(vec![0]);

    Ok("Ok".into())
}


async fn logout(session: Session) -> Result<HttpResponse>
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


#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let bind = "0.0.0.0:8080";
    println!("Starting server at: {}", &bind);

    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    HttpServer::new(move ||
        {
            App::new()
                .wrap(RedisSession::new("127.0.0.1:6379", &private_key))
                .service(scope("/auth")
                    .route("", get().to(auth))
                    .route("/login", get().to(login))
                    .route("/logout", get().to(logout))
                    .route("/{filepath:.*}", get().to(files))
                )
                .service(scope("/")
                    .wrap(CheckLogin)
                    .route("", get().to(index))
                    .route("/{filepath:.*}", get().to(files))
                )
                // .service(Files::new("", "./web_layout")
                //     .index_file("index.html"))
        })
        .bind(&bind)?
        .run()
        .await
}
