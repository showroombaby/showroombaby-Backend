use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Offer {
    title: String,
    description: String,
    price: f64,
}

#[get("/search")]
async fn search(_query: web::Query<String>) -> impl Responder {
    let offers = vec![Offer {
        title: "Offre 1 exemple blabla".into(),
        description: "Description 1".into(),
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
