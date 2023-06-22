use std::{path::PathBuf, str::FromStr};

use actix_web::{middleware, App, HttpServer, web, guard};
use bundlr_sdk::{BundlrBuilder, currency::solana::{Solana, SolanaBuilder}};
use data_encoding::BASE64URL_NOPAD;
use reqwest::Url;

use crate::api::routes::upload;

use self::routes::index;

use sha2::Digest;

mod routes;

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index::index_route)))
            .service(web::resource("/record")
            .guard(guard::Header("X-API-Key", "76793ab1-dd7b-4c67-bf67-24f0deb32446"))
            .route(web::post().to(upload::upload_route))
        )
    })
    .bind("127.0.0.1:8080")?
    .workers(2) // TODO: make this configurable
    .run()
    .await
}