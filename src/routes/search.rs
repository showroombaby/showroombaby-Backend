use actix_web::{get, web, Responder};
use diesel::prelude::*;
use crate::models::Offer;
use crate::schema::offers::dsl::*;
use crate::db::establish_connection;

#[derive(Deserialize)]
struct SearchParams {
    query: Option<String>,
    category: Option<String>,
}

#[get("/search")]
pub async fn search(params: web::Query<SearchParams>) -> impl Responder {
    let conn = establish_connection();

    let mut query = offers.into_boxed();

    if let Some(search_term) = &params.query {
        query = query.filter(title.like(format!("%{}%", search_term)));
    }

    if let Some(cat) = &params.category {
        query = query.filter(category.eq(cat));
    }

    let results = query
        .load::<Offer>(&conn)
        .expect("Erreur lors du chargement des offres");

    web::Json(results)
}
