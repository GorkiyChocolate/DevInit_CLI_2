use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ComposeService {
    pub name: String,
    pub image: String,
    pub ports: Vec<String>,
    pub environment: Vec<String>,
    pub volumes: Vec<String>,
    pub networks: Vec<String>,
    pub depends_on: Vec<String>,
    pub restart: Option<String>,
    pub command: Option<Vec<String>>,
}