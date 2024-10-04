use actix_web::{web, HttpResponse, Responder};
use aws_sdk_s3::Client as S3Client;
use crate::config::Config;
use crate::s3_service;

pub async fn check_s3_bucket(
    s3_client: web::Data<S3Client>,
    config: web::Data<Config>,
) -> impl Responder {
    match s3_service::check_bucket_exists(&s3_client, &config.s3_bucket).await {
        Ok(true) => HttpResponse::Ok().body(format!("Le bucket '{}' existe.", config.s3_bucket)),
        Ok(false) => HttpResponse::NotFound().body(format!("Le bucket '{}' n'existe pas.", config.s3_bucket)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Erreur lors de la vérification du bucket : {:?}", e)),
    }
}
