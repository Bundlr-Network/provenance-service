use actix_web::{HttpServer, App, web, HttpResponse, middleware};
use api::run_server;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run_server().await
}
