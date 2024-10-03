mod jwt;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder, HttpRequest, Error};
use jwt::validate_token;
use serde::{Deserialize, Serialize};

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
async fn search(params: web::Query<SearchParams>,req: HttpRequest,) -> Result<impl Responder, Error> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        let token = auth_header.to_str().unwrap().trim_start_matches("Bearer ");
        match validate_token(token) {
            Ok(token_data) => {
                let query = &params.query;
                let offers = vec![Offer {
                    title: format!("Résultat pour {}", query),
                    description: format!("Bonjour, {}!", token_data.claims.sub),
                    price: 100.0,
                }];
                Ok(web::Json(offers))
            }
            Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid Token")),
        }
    } else {
        Err(actix_web::error::ErrorUnauthorized("Missing Token"))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
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
