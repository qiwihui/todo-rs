use crate::handlers;
use actix_web::web;
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::hello)
        .route("/todos{_:/?}", web::get().to(handlers::todos))
        .route("/todos{_:/?}", web::post().to(handlers::create_todo))
        .route("/todos/{list_id}{_:/?}", web::get().to(handlers::todo))
        .route(
            "/todos/{list_id}/items{_:/?}",
            web::get().to(handlers::items),
        )
        .route(
            "/todos/{list_id}/items{_:/?}",
            web::post().to(handlers::create_item),
        )
        .route(
            "/todos/{list_id}/items/{item_id}{_:/?}",
            web::get().to(handlers::get_item),
        )
        .route(
            "/todos/{list_id}/items/{item_id}{_:/?}",
            web::put().to(handlers::check_todo),
        );
}
