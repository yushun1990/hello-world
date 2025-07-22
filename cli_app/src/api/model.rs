use serde::Deserialize;

use crate::model::PostStatus;

#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub author_id: i64,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub status: PostStatus,
}

#[derive(Deserialize)]
pub struct UpdatePostRequest {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub status: PostStatus,
}
