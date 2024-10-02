use diesel::prelude::*;
use crate::models::Offer;

pub fn search_offers(term: &str, conn: &MysqlConnection) -> Vec<Offer> {
    use crate::schema::offers::dsl::*;
    offers.filter(title.like(format!("%{}%", term)))
        .load::<Offer>(conn)
        .expect("Erreur lors du chargement des offres")
}
