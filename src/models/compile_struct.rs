use crate::models::{
    cicd_struct::Pipeline, docker_compose_struct::RecipeCompose, k8s_struct::Deployment,
    services_struct::Services,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileSpec {
    pub services: Vec<CompileService>,
    pub kubernetes: Option<Deployment>,
    pub cicd: Option<Pipeline>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompileService {
    BuiltIn(Services),
    Custom(Box<RecipeCompose>),
}

impl CompileService {
    pub fn to_recipe(&self) -> RecipeCompose {
        match self {
            Self::BuiltIn(service) => service.to_recipe(),
            Self::Custom(recipe) => recipe.as_ref().clone(),
        }
    }
}
