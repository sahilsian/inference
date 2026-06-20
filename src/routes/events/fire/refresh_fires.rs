use actix_web::{HttpResponse};

pub async fn refresh_fires() -> HttpResponse {
    HttpResponse::Ok().finish()
}   