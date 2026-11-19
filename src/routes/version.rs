use actix_web::{HttpResponse, Responder, get};

#[get("/version")]
pub async fn version() -> impl Responder {
    HttpResponse::Ok().body("dev")
}