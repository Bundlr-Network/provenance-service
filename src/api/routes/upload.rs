use std::str::FromStr;

use actix_web::HttpResponse;
use bundlr_sdk::{BundlrBuilder, currency::solana::{Solana, SolanaBuilder}};
use data_encoding::BASE64URL_NOPAD;
use reqwest::Url;
use serde_json::json;
use sha2::Digest;

pub async fn upload_route() -> HttpResponse {
    let url = "https://node2.bundlr.network";
    let currency = SolanaBuilder::new().wallet(
        "kNykCXNxgePDjFbDWjPNvXQRa8U12Ywc19dFVaQ7tebUj3m7H4sF4KKdJwM7yxxb3rqxchdjezX9Szh8bLcQAjb")
        .build()
        .expect("Could not create Solana instance");
    let mut bundlr = BundlrBuilder::<Solana>::new()
        .url(Url::from_str(url).unwrap())
        .currency(currency)
        .fetch_pub_info()
        .await.unwrap()
        .build()
        .unwrap();

    let mut tx = bundlr.create_transaction(vec![], vec![]).unwrap();

    bundlr.sign_transaction(&mut tx).await.unwrap();
    
    let mut hasher = sha2::Sha256::new();
    hasher.update(&tx.get_signarure()[..]);
    let id = BASE64URL_NOPAD.encode(&hasher.finalize());

    bundlr.send_transaction(tx).await.unwrap();

    HttpResponse::Ok().body(serde_json::json!({ "id": id }).to_string())
}