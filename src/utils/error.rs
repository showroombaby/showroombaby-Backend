use actix_web::{error, HttpResponse};

pub fn handle_error() -> HttpResponse {
    HttpResponse::InternalServerError().json("Une erreur est survenue")
}
