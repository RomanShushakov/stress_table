use actix_web::{HttpServer, App, middleware};
use actix_files::{Files};


#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    std::env::set_var("RUST_LOG", "actix_web=info");

    let bind = "0.0.0.0:8080";
    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move ||
        {
            App::new()
                .wrap(middleware::Logger::default())
                .service(Files::new("", "./web_layout")
                    .index_file("index.html"))
        })
        .bind(&bind)?
        .run()
        .await
}

