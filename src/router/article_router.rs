use salvo::prelude::*;

use crate::controller::article_controller::*;

pub fn init_router() -> Router {
    Router::new().push(
        Router::with_path("articles")
            .get(get_article_list)
            .post(create_article)
            .push(
                Router::with_path("<id>")
                    .get(get_article)
                    .put(update_article),
            ),
    )
}
