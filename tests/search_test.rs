#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_search() {
        let app = test::init_service(App::new().service(search)).await;
        let req = test::TestRequest::get().uri("/search?query=voiture").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
