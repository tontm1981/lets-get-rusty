use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    title: String,
    content: String,
}

impl Post {
    pub fn new(title: String, content: String) -> Post {
        Post { title, content }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}