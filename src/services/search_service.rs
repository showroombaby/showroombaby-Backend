use crate::models::Offer;
use diesel::prelude::*;

pub fn search_offers(term: &str, conn: &MysqlConnection) -> Vec<Offer> {
    use crate::schema::offers::dsl::*;
    offers
        .filter(title.like(format!("%{}%", term)))
        .load::<Offer>(conn)
        .expect("Erreur lors du chargement des offres")
}

// Tests unitaires

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::prelude::*;
    use crate::models::Offer;

    #[test]
    fn test_search_offers() {
        let conn = establish_connection();
        let results = search_offers("voiture", &conn);
        assert!(results.len() > 0);
    }
}
