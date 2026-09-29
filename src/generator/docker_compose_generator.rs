use crate::errors::GenerationError;
use crate::generator::writer::write_yaml;
use crate::models::compile_struct::CompileService;
use crate::models::docker_compose_struct::RecipeCompose;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub fn generate_docker_compose(
    services: &[CompileService],
    output_dir: &Path,
) -> Result<Vec<PathBuf>, GenerationError> {
    let mut compose_services = BTreeMap::new();
    for service in services {
        let recipe = service.to_recipe();
        compose_services.insert(recipe.name.clone(), ComposeService::from_recipe(&recipe));
    }

    let compose = DockerComposeFile {
        services: compose_services,
    };
    let path = write_yaml(&compose, &output_dir.join("docker-compose.yaml"))?;
    Ok(vec![path])
}

#[derive(Debug, Serialize)]
struct DockerComposeFile {
    services: BTreeMap<String, ComposeService>,
}

#[derive(Debug, Serialize)]
struct ComposeService {
    image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    networks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    depends_on: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restart: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<BTreeMap<String, String>>,
}

impl ComposeService {
    fn from_recipe(recipe: &RecipeCompose) -> Self {
        let labels = recipe.description.as_ref().map(|description| {
            BTreeMap::from([("com.devinit.description".to_string(), description.clone())])
        });
        Self {
            image: recipe.image.clone(),
            ports: recipe.ports.clone(),
            environment: recipe.environment.clone(),
            volumes: recipe.volumes.clone(),
            networks: recipe.networks.clone(),
            depends_on: recipe.depends_on.clone(),
            restart: recipe.restart.clone(),
            command: recipe.command.clone(),
            labels,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::services_struct::Services;

    #[test]
    fn generates_selected_services_with_descriptions() {
        let output =
            std::env::temp_dir().join(format!("devinit-compose-test-{}", std::process::id()));
        let services = vec![
            CompileService::BuiltIn(Services::PostgreSQL),
            CompileService::BuiltIn(Services::Redis),
        ];

        let paths = generate_docker_compose(&services, &output).expect("generation should succeed");
        assert_eq!(paths, vec![output.join("docker-compose.yaml")]);
        let contents = std::fs::read_to_string(&paths[0]).expect("compose file should exist");
        assert!(contents.contains("postgresql:"));
        assert!(contents.contains("redis:"));
        assert!(contents.contains("com.devinit.description"));
        let _ = std::fs::remove_dir_all(output);
    }
}
