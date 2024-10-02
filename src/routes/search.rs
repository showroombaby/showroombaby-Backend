use rocket::serde::json::Json;
use crate::models::Offer;
use crate::services::search_service::search_offers;
use crate::db::establish_connection;

#[get("/search?<query>")]
pub fn search(query: String) -> Json<Vec<Offer>> {
    let conn = establish_connection();
    let results = search_offers(&query, &conn);
    Json(results)
}
