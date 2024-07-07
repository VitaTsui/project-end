use rbatis::executor::Executor;
use rbatis::html_sql;
use rbatis::rbdc::db::ExecResult;

use crate::model::dto::article_dto::{ArticleCreateDto, ArticleSearchDto, ArticleUpdateDto};
use crate::model::entity::article_entity::ArticleEntity;

pub struct ArticleMapper;

impl ArticleMapper {
    #[html_sql("src/mapper/xml/article_xml.html")]
    pub async fn get_article_list(
        rb: &mut dyn Executor,
        search: &ArticleSearchDto,
    ) -> rbatis::Result<Vec<ArticleEntity>> {
        impled!()
    }

    #[html_sql("src/mapper/xml/article_xml.html")]
    pub async fn get_article_count(
        rb: &mut dyn Executor,
        search: &ArticleSearchDto,
    ) -> rbatis::Result<i64> {
        impled!()
    }

    #[html_sql("src/mapper/xml/article_xml.html")]
    pub async fn get_article_by_id(
        rb: &mut dyn Executor,
        id: i64,
    ) -> rbatis::Result<ArticleEntity> {
        impled!()
    }

    #[html_sql("src/mapper/xml/article_xml.html")]
    pub async fn create_article(
        rb: &mut dyn Executor,
        article: &ArticleCreateDto,
    ) -> rbatis::Result<ExecResult> {
        impled!()
    }

    #[html_sql("src/mapper/xml/article_xml.html")]
    pub async fn update_article_by_id(
        rb: &mut dyn Executor,
        article: &ArticleUpdateDto,
    ) -> rbatis::Result<ExecResult> {
        impled!()
    }
}
