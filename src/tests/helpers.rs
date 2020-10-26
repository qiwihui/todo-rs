use crate::config::init_pool;
use crate::config::Config;
use crate::models;
use crate::routes::routes;
use actix_web::dev::ServiceResponse;
use actix_web::{test, App};
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Serialize;

lazy_static! {
    pub static ref APP_STATE: models::AppState = {
        dotenv().ok();
        let config = Config::from_env().unwrap();
        let pool = init_pool(&config).unwrap();
        models::AppState { pool: pool.clone() }
    };
}

pub async fn test_get(route: &str) -> ServiceResponse {
    let mut app = test::init_service(App::new().data(APP_STATE.clone()).configure(routes)).await;
    test::call_service(&mut app, test::TestRequest::get().uri(route).to_request()).await
}

pub async fn assert_get(route: &str) -> ServiceResponse {
    let response = test_get(route).await;
    assert!(response.status().is_success());
    response
}

pub async fn test_post<T: Serialize>(route: &str, params: T) -> ServiceResponse {
    let mut app = test::init_service(App::new().data(APP_STATE.clone()).configure(routes)).await;
    test::call_service(
        &mut app,
        test::TestRequest::post()
            .set_json(&params)
            .uri(route)
            .to_request(),
    )
    .await
}

pub async fn assert_post<T: Serialize>(route: &str, params: T) -> ServiceResponse {
    let response = test_post(route, params).await;
    assert!(response.status().is_success());
    response
}
