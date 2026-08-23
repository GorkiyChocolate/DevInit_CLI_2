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

/*
   Example
   devinit add redis
   {
       "name": "redis",
       "description": "Redis cache",
       "image": "redis:8",
       "ports": ["6379:6379"],
       "volumes": ["redis_data:/data"],
       "restart": "unless-stopped"
   }
   devinit add postgresql
   {
       "name": "postgres",
       "image": "postgres:17",
       "ports": ["5432:5432"],
       "environment": [
           "POSTGRES_DB=app",
           "POSTGRES_USER=admin",
           "POSTGRES_PASSWORD=password"
       ],
       "volumes": [
           "postgres_data:/var/lib/postgresql/data"
       ],
       "restart": "unless-stopped"
   }
   devinit add prometheus
   {
       "name": "prometheus",
       "image": "prom/prometheus",
       "ports": ["9090:9090"],
       "files": [
           {
           "path": "prometheus.yml",
           "content": "..."
           }
       ]
   }
*/
