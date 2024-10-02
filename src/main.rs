use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Offer {
    title: String,
    description: String,
    price: f64,
}

#[derive(Deserialize)]
struct SearchParams {
    query: String,
}

#[get("/search")]
async fn search(params: web::Query<SearchParams>) -> impl Responder {
    let query = &params.query;
    let offers = vec![Offer {
        title: format!("Résultat pour {}", query),
        description: "Description d'exemple".into(),
        price: 100.0,
    }];
    web::Json(offers)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .service(search)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
