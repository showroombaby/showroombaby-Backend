use actix_web::{post, web, HttpResponse, Responder};
use crate::services::dynamodb_service::{add_offer, Offer};

#[post("/offers")]
async fn create_offer(offer: web::Json<Offer>) -> impl Responder {
    match add_offer(offer.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Votre offre a bien été ajoutée."),
        Err(e) => HttpResponse::InternalServerError().json(format!("Erreur lors de l'ajout de l'offre : {}", e)),
    }
}
