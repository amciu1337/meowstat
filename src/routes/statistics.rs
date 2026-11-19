use actix_web::{HttpResponse, Responder, get};
use crate::monitor::get_machine_statistics;

#[get("/statistics")]
pub async fn statistics() -> impl Responder {
    let stats = get_machine_statistics().await.unwrap();
    HttpResponse::Ok().json(stats)
}