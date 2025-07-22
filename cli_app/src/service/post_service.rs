use std::collections::HashMap;

use tokio::sync::Mutex;

use crate::{
    api::model::{CreatePostRequest, UpdatePostRequest},
    model::Post,
};

pub struct InMemoryPostStore {
    pub counter: i64,
    pub items: HashMap<i64, Post>,
}

pub struct InMemoryPostService {
    data: Mutex<InMemoryPostStore>,
}

impl InMemoryPostService {}

impl Default for InMemoryPostService {
    fn default() -> Self {
        Self {
            data: Mutex::new(InMemoryPostStore {
                counter: 0,
                items: Default::default(),
            }),
        }
    }
}

#[allow(async_fn_in_trait)]
pub trait PostService {
    async fn get_all_posts(&self) -> anyhow::Result<Vec<Post>>;
    async fn get_post_by_id(&self, id: i64) -> anyhow::Result<Post>;
    async fn get_post_by_slug(&self, slug: &str) -> anyhow::Result<Post>;
    async fn create_post(&self, req: CreatePostRequest) -> anyhow::Result<Post>;
    async fn update_post(&self, id: i64, req: UpdatePostRequest) -> anyhow::Result<Post>;
    async fn delete_post(&self, id: i64) -> anyhow::Result<()>;
}

impl PostService for InMemoryPostService {
    async fn get_all_posts(&self) -> anyhow::Result<Vec<Post>> {
        let data = self.data.lock().await;
        Ok(data.items.values().map(|post| (*post).clone()).collect())
    }

    async fn get_post_by_id(&self, id: i64) -> anyhow::Result<Post> {
        let data = self.data.lock().await;
        match data.items.get(&id) {
            Some(post) => Ok((*post).clone()),
            None => anyhow::bail!("Post not found: {}", id),
        }
    }

    async fn get_post_by_slug(&self, slug: &str) -> anyhow::Result<Post> {
        let data = self.data.lock().await;

        for (_, post) in data.items.iter() {
            if post.slug == slug {
                return Ok(post.clone());
            }
        }

        anyhow::bail!("Post not found with slug: {}", slug)
    }

    async fn create_post(&self, req: CreatePostRequest) -> anyhow::Result<Post> {
        let mut data = self.data.lock().await;

        data.counter += 1;
        let ts = chrono::offset::Utc::now();

        let post = Post {
            id: data.counter,
            author_id: req.author_id,
            slug: req.slug,
            title: req.title,
            content: req.content,
            status: req.status,
            created: ts,
            updated: ts,
        };

        data.items.insert(post.id, post);

        self.get_post_by_id(data.counter).await
    }

    async fn update_post(&self, id: i64, req: UpdatePostRequest) -> anyhow::Result<Post> {
        let mut data = self.data.lock().await;

        let mut post = data
            .items
            .get_mut(&id)
            .ok_or(anyhow::bail!("Post not found: {}", id))?;

        post.slug = req.slug;
        post.title = req.title;
        post.content = req.content;
        post.status = req.status;

        self.get_post_by_id(id).await
    }

    async fn delete_post(&self, id: i64) -> anyhow::Result<()> {
        let mut data = self.data.lock().await;

        match data.items.remove(&id) {
            None => {
                anyhow::bail!("Post not found: {}", id)
            }
            Some(_) => Ok(()),
        }
    }
}
