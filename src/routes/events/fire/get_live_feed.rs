use actix_web::{HttpResponse};

pub async fn get_live_feed() -> HttpResponse {
    HttpResponse::Ok().finish()
}   