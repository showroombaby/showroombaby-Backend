use actix_web::{App, test, web};
use actix_web::http::StatusCode;
use crate::routes::search;
use actix_web::dev::Service;

#[actix_web::test]
async fn test_search_route() {
    let app = test::init_service(App::new().service(search)).await;
    let req = test::TestRequest::get().uri("/search?query=voiture").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
