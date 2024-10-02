use actix_web::{get, web, Responder};
use diesel::prelude::*;
use crate::models::Offer;
use crate::schema::offers::dsl::*;
use crate::db::establish_connection;

#[derive(Deserialize)]
struct SearchParams {
    query: String,
}

#[get("/search")]
pub async fn search(params: web::Query<SearchParams>) -> impl Responder {
    let conn = establish_connection();
    let query = &params.query;

    let results = offers
        .filter(title.like(format!("%{}%", query)))
        .load::<Offer>(&conn)
        .expect("Erreur lors du chargement des offres");

    web::Json(results)
}
