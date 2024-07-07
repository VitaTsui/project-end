use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArticleEntity {
    pub id: i64,
    pub tags: Vec<String>,
    pub title: String,
    pub intro: String,
    pub article: String,
    pub status: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
