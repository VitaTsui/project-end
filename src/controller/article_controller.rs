use salvo::prelude::*;
use tracing::error as tracing_error;

use crate::{
    model::dto::article_dto::{ArticleCreateDto, ArticleSearchDto, ArticleUpdateDto},
    service::article_service::ArticleService,
    utils::res::{error, success, PaginatedResponse},
};

#[handler]
pub async fn get_article_list(req: &mut Request, res: &mut Response) {
    let queries = req.parse_queries();

    match queries {
        Ok(_) => {
            let search: ArticleSearchDto = queries.unwrap();

            let (articles, total) = ArticleService::get_article_list(ArticleSearchDto {
                page_num: search.page_num.map_or(Some(1), |page| Some(page - 1)),
                ..search.clone()
            })
            .await;

            match search {
                ArticleSearchDto {
                    page_num: Some(_),
                    page_size: Some(_),
                    ..
                } => match articles {
                    Ok(articles) => {
                        res.render(success(PaginatedResponse {
                            list: articles,
                            page_num: search.page_num.unwrap(),
                            page_size: search.page_size.unwrap(),
                            total: total.unwrap(),
                        }));
                    }
                    Err(e) => {
                        tracing_error!("{:?}", e);
                        res.render(error("获取文章分页列表失败"));
                    }
                },
                _ => match articles {
                    Ok(articles) => {
                        res.render(success(articles));
                    }
                    Err(e) => {
                        tracing_error!("{:?}", e);
                        res.render(error("获取文章列表失败"));
                    }
                },
            }
        }
        Err(e) => {
            tracing_error!("{:?}", e);
            res.render(error("参数解析失败"));
            return;
        }
    }
}

#[handler]
pub async fn get_article(req: &mut Request, res: &mut Response) {
    let id: Option<i64> = req.param("id");

    match id {
        Some(id) => {
            let article = ArticleService::get_article(id).await;

            match article {
                Ok(article) => {
                    res.render(success(article));
                }
                Err(e) => {
                    tracing_error!("{:?}", e);
                    res.render(error("获取文章详情失败"));
                }
            }
        }
        None => {
            res.render(error("缺少文章id"));
        }
    }
}

#[handler]
pub async fn create_article(req: &mut Request, res: &mut Response) {
    let article: Result<ArticleCreateDto, _> = req.parse_body().await;

    match article {
        Ok(article) => {
            let _artice = ArticleCreateDto::new(
                article.tags,
                article.title,
                article.intro,
                article.article,
                article.status,
            );
            let is_created = ArticleService::create_article(&_artice).await;

            match is_created {
                Ok(_) => {
                    res.render(success(()));
                }
                Err(e) => {
                    tracing_error!("{:?}", e);
                    res.render(error(e));
                }
            }
        }
        Err(e) => {
            tracing_error!("{:?}", e);
            res.render(error("解析文章失败"));
        }
    }
}

#[handler]
pub async fn update_article(req: &mut Request, res: &mut Response) {
    let id: Option<i64> = req.param("id");

    match id {
        Some(id) => {
            let article: Result<ArticleUpdateDto, _> = req.parse_body().await;

            match article {
                Ok(article) => {
                    let _artice = ArticleUpdateDto::new(
                        id,
                        article.tags,
                        article.title,
                        article.intro,
                        article.article,
                        article.status,
                    );
                    let is_updated = ArticleService::update_article(&_artice).await;

                    match is_updated {
                        Ok(_) => {
                            res.render(success(()));
                        }
                        Err(e) => {
                            tracing_error!("{:?}", e);
                            res.render(error(e));
                        }
                    }
                }
                Err(e) => {
                    tracing_error!("{:?}", e);
                    res.render(error("解析文章失败"));
                }
            }
        }
        None => {
            res.render(error("缺少文章id"));
        }
    }
}
