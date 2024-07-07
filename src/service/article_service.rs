use crate::{
    mapper::article_mapper::ArticleMapper,
    model::{
        dto::article_dto::{ArticleCreateDto, ArticleSearchDto, ArticleUpdateDto},
        entity::article_entity::ArticleEntity,
    },
    utils::func::is_ok,
    RB,
};

pub struct ArticleService;

impl ArticleService {
    pub async fn get_article_list(
        search: ArticleSearchDto,
    ) -> (rbatis::Result<Vec<ArticleEntity>>, rbatis::Result<i64>) {
        let articles = ArticleMapper::get_article_list(&mut RB.clone(), &search).await;
        let total = ArticleMapper::get_article_count(&mut RB.clone(), &search).await;

        (articles, total)
    }

    pub async fn get_article(id: i64) -> rbatis::Result<ArticleEntity> {
        ArticleMapper::get_article_by_id(&mut RB.clone(), id).await
    }

    pub async fn create_article(article: &ArticleCreateDto) -> Result<(), &str> {
        let created = ArticleMapper::create_article(&mut RB.clone(), article).await;

        is_ok(created, "创建文章失败")
    }

    pub async fn update_article(article: &ArticleUpdateDto) -> Result<(), &str> {
        let created = ArticleMapper::update_article_by_id(&mut RB.clone(), article).await;

        is_ok(created, "更新文章失败")
    }
}
