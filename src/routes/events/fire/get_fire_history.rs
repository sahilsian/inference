use actix_web::{HttpResponse};

pub async fn get_fire_history() -> HttpResponse {
    HttpResponse::Ok().finish()
}   