use actix_web::HttpResponse;

pub async fn index_route() -> HttpResponse {
    HttpResponse::Ok().body("Get on Rail!")
}