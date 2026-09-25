use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecipeCompose {
    pub name: String,
    pub description: Option<String>,

    // Docker Compose
    pub image: String,
    pub ports: Option<Vec<String>>,
    pub environment: Option<Vec<String>>,
    pub volumes: Option<Vec<String>>,
    pub networks: Option<Vec<String>>,
    pub depends_on: Option<Vec<String>>,
    pub restart: Option<String>,
    pub command: Option<Vec<String>>,

    pub files: Option<File>,

    // .env
    pub env: Option<Vec<String>>,

    pub notes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigsList {
    pub configs: Vec<RecipeCompose>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub path: String,
    pub content: String,
}
