use actix_web::{HttpServer, App, middleware};
use actix_files::{Files, NamedFile};
use actix_web::dev::{ServiceRequest, ServiceResponse};


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
                .service(Files::new("/", "./web_layout")
                    .index_file("index.html")
                    .default_handler(|req: ServiceRequest|
                        {
                            let (http_req, _payload) = req.into_parts();
                            async {
                                let response =
                                    NamedFile::open("./web_layout/index.html")?
                                    .into_response(&http_req)?;
                                Ok(ServiceResponse::new(http_req, response))
                            }
                        }),
                )
        })
    .bind(&bind)?
    .run()
    .await
}
