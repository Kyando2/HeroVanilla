use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_files::Files;
mod utils;
mod get;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/images", "static/images/").show_files_listing())
            .service(Files::new("/", "./static/root/").index_file("index.html"))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

