use actix_web::{HttpServer, App, Result, HttpResponse, HttpRequest};
use actix_web::web::{get, post, scope};
use actix_files::{NamedFile};
use actix_redis::RedisSession;
use actix_session::Session;
use std::path::PathBuf;
use rand::Rng;
use tokio_postgres::NoTls;
use askama::Template;


mod check_login;
use check_login::CheckLogin;

mod config;

mod models;

mod errors;

mod db;

mod handlers;
use handlers::{register, login, logout, update_cache, load_cache};

mod templates;


async fn index(_req: HttpRequest, session: Session) -> Result<HttpResponse>
{
    let username: String = session.get::<String>("username").unwrap().unwrap();
    let index = templates::AuthorizedUserInfo { username: &username }.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(index))
}


async fn files(req: HttpRequest, _session: Session) -> Result<NamedFile>
{
    let mut path = PathBuf::new();
    let prefix_path = "./web_layout";
    let requested_path: PathBuf = req.match_info().query("file_path").parse().unwrap();
    path.push(prefix_path);
    path.push(requested_path);
    Ok(NamedFile::open(path)?)
}


async fn auth(_req: HttpRequest, _session: Session) -> Result<NamedFile>
{
    let auth_path: PathBuf = "./web_layout/auth.html".parse().unwrap();
    Ok(NamedFile::open(auth_path)?)
}


#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    dotenv::dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    let redis_addr = config.redis_addr;

    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    let server = HttpServer::new(move ||
        {
            App::new()
                .data(pool.clone())
                .wrap(RedisSession::new(&redis_addr, &private_key).ttl(259200))
                .service(scope("/cache")
                    .wrap(CheckLogin)
                    .route("/update", post().to(update_cache))
                    .route("/load", get().to(load_cache))
                )
                .service(scope("/auth")
                    .route("", get().to(auth))
                    .route("/register", post().to(register))
                    .route("/login", post().to(login))
                    .route("/logout", post().to(logout))
                    .route("/{file_path:.*}", get().to(files))
                )
                .service(scope("/")
                    .wrap(CheckLogin)
                    .route("", get().to(index))
                    .route("/{file_path:.*}", get().to(files))
                )
                // .service(Files::new("", "./web_layout")
                //     .index_file("index.html"))
        })
        .bind(config.server_addr.clone())?
        .run();

    println!("Server running at http://{}/", config.server_addr);

    server.await
}
