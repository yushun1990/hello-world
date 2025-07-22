use serde::Serialize;

use crate::model::Post;

#[derive(Serialize)]
pub struct SinglePostResponse {
    pub data: Post,
}

#[derive(Serialize)]
pub struct ListPostResponse {
    pub data: Vec<Post>,
}
