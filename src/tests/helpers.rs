use crate::routes::routes;
use actix_web::dev::ServiceResponse;
use actix_web::{test, App};
// 测试get
pub async fn test_get(route: &str) -> ServiceResponse {
    let mut app = test::init_service(App::new().configure(routes)).await;
    test::call_service(&mut app, test::TestRequest::get().uri(route).to_request()).await
}

pub async fn assert_get(route: &str) -> ServiceResponse {
    let response = test_get(route).await;
    assert!(response.status().is_success());
    response
}
