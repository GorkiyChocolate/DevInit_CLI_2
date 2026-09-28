use crate::models::{
    cicd_struct::Pipeline, docker_compose_struct::RecipeCompose, k8s_struct::Deployment,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileSpec {
    pub services: Vec<RecipeCompose>,
    pub kubernetes: Option<Deployment>,
    pub cicd: Option<Pipeline>,
}
