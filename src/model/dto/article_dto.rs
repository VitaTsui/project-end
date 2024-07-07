use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArticleCreateDto {
    pub tags: Vec<String>,
    pub title: String,
    pub intro: String,
    pub article: String,
    pub status: i64,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}
impl ArticleCreateDto {
    pub fn new(
        tags: Vec<String>,
        title: String,
        intro: String,
        article: String,
        status: i64,
    ) -> Self {
        ArticleCreateDto {
            tags,
            title,
            intro,
            article,
            status,
            created_at: Some(DateTime::utc()),
            updated_at: Some(DateTime::utc()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArticleUpdateDto {
    pub id: Option<i64>,
    pub tags: Option<Vec<String>>,
    pub title: Option<String>,
    pub intro: Option<String>,
    pub article: Option<String>,
    pub status: Option<i64>,
    pub updated_at: Option<DateTime>,
}
impl ArticleUpdateDto {
    pub fn new(
        id: i64,
        tags: Option<Vec<String>>,
        title: Option<String>,
        intro: Option<String>,
        article: Option<String>,
        status: Option<i64>,
    ) -> Self {
        ArticleUpdateDto {
            id: Some(id),
            tags,
            title,
            intro,
            article,
            status,
            updated_at: Some(DateTime::utc()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArticleSearchDto {
    pub title: Option<String>,
    pub tag: Option<String>,
    pub status: Option<i64>,
    pub created_at: Option<Vec<String>>,
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
}
