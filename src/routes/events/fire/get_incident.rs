use actix_web::{HttpResponse};

pub async fn get_incident() -> HttpResponse {
    HttpResponse::Ok().finish()
}   