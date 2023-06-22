use std::str::FromStr;

use actix_web::{HttpResponse, web::Query};
use bundlr_sdk::{BundlrBuilder, currency::solana::{Solana, SolanaBuilder}, tags::Tag};
use data_encoding::BASE64URL_NOPAD;
use reqwest::Url;
use serde::Deserialize;
use sha2::Digest;

#[derive(Deserialize)]
pub struct RecordQuery { 
    hash: Option<String>, 
    uploaded_for: Option<String>,
    prompt: Option<String>,
    prompt_hash: Option<String>,
    model: Option<String>
 } 

pub async fn upload_route(params: Query<RecordQuery>) -> HttpResponse {
    let record = params.into_inner();
    
    let url = "https://node2.bundlr.network";
    let currency = SolanaBuilder::new().wallet(
        "kNykCXNxgePDjFbDWjPNvXQRa8U12Ywc19dFVaQ7tebUj3m7H4sF4KKdJwM7yxxb3rqxchdjezX9Szh8bLcQAjb")
        .build()
        .expect("Could not create Solana instance");
    let bundlr = BundlrBuilder::<Solana>::new()
        .url(Url::from_str(url).unwrap())
        .currency(currency)
        .fetch_pub_info()
        .await.unwrap()
        .build()
        .unwrap();

    let mut tags = vec![
        Tag::new("Data-Protocol", "Provenance-Confimation"),
        Tag::new("Hashing-Algo", "sha256"),
        Tag::new("Data-Hash", &record.hash.unwrap_or("SHA-256".to_string())),
    ];

    if record.uploaded_for.is_some() { tags.push(Tag::new("Uploaded-For", &record.uploaded_for.unwrap())); };
    if record.prompt.is_some() { tags.push(Tag::new("Prompt", &record.prompt.unwrap())); };
    if record.prompt_hash.is_some() { tags.push(Tag::new("Prompt-Hash", &record.prompt_hash.unwrap())); };
    if record.model.is_some() { tags.push(Tag::new("Model", &record.model.unwrap())); };

    let mut tx = bundlr.create_transaction(vec![], tags).unwrap();

    bundlr.sign_transaction(&mut tx).await.unwrap();
    
    let mut hasher = sha2::Sha256::new();
    hasher.update(&tx.get_signarure()[..]);
    let id = BASE64URL_NOPAD.encode(&hasher.finalize());

    bundlr.send_transaction(tx).await.unwrap();

    HttpResponse::Ok().body(serde_json::json!({ "id": id }).to_string())
}