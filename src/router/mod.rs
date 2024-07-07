use salvo::Router;

mod article_router;

pub fn router() -> Router {
    Router::new().push(Router::with_path("api").push(article_router::init_router()))
}
