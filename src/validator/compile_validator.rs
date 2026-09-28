use crate::errors::ValidationError;
use crate::models::compile_struct::CompileSpec;
use std::collections::{HashMap, HashSet};

pub fn validate_compile(spec: &CompileSpec) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    for service in &spec.services {
        crate::validator::services_validator::validate_service(&service.name, service, &mut errors);
    }
    if let Some(kubernetes) = &spec.kubernetes {
        crate::validator::k8s_validator::validate_deployment(kubernetes, &mut errors);
    }
    if let Some(cicd) = &spec.cicd {
        crate::validator::cicd_validator::validate_pipeline(cicd, &mut errors);
    }
    validate_cross_references(spec, &mut errors);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_cross_references(spec: &CompileSpec, errors: &mut Vec<ValidationError>) {
    let mut services = HashMap::new();
    for (index, service) in spec.services.iter().enumerate() {
        if services.insert(service.name.as_str(), index).is_some() {
            errors.push(ValidationError {
                path: format!("services[{index}].name"),
                message: format!("service '{}' is defined more than once", service.name),
            });
        }
    }
    for (index, service) in spec.services.iter().enumerate() {
        if let Some(dependencies) = &service.depends_on {
            for (dependency_index, dependency) in dependencies.iter().enumerate() {
                if !services.contains_key(dependency.as_str()) {
                    errors.push(ValidationError {
                        path: format!("services[{index}].depends_on[{dependency_index}]"),
                        message: format!("service '{dependency}' does not exist"),
                    });
                }
            }
        }
    }
    detect_cycles(spec, &services, errors);
}

fn detect_cycles(
    spec: &CompileSpec,
    services: &HashMap<&str, usize>,
    errors: &mut Vec<ValidationError>,
) {
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    for index in 0..spec.services.len() {
        if visit_service(index, spec, services, &mut visiting, &mut visited) {
            errors.push(ValidationError {
                path: format!("services[{index}].depends_on"),
                message: "circular dependency detected".to_string(),
            });
        }
    }
}

fn visit_service(
    index: usize,
    spec: &CompileSpec,
    services: &HashMap<&str, usize>,
    visiting: &mut HashSet<usize>,
    visited: &mut HashSet<usize>,
) -> bool {
    if visiting.contains(&index) {
        return true;
    }
    if !visited.insert(index) {
        return false;
    }
    visiting.insert(index);
    let cycle = spec.services[index]
        .depends_on
        .as_ref()
        .is_some_and(|dependencies| {
            dependencies
                .iter()
                .filter_map(|name| services.get(name.as_str()))
                .any(|dependency| visit_service(*dependency, spec, services, visiting, visited))
        });
    visiting.remove(&index);
    cycle
}

pub fn validate_compiler() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::docker_compose_struct::RecipeCompose;

    fn service(name: &str, image: &str, depends_on: Option<Vec<String>>) -> RecipeCompose {
        RecipeCompose {
            name: name.to_string(),
            description: None,
            image: image.to_string(),
            ports: None,
            environment: None,
            volumes: None,
            networks: None,
            depends_on,
            restart: None,
            command: None,
            files: None,
            env: None,
            notes: None,
        }
    }

    #[test]
    fn reports_empty_image_and_invalid_port() {
        let mut invalid = service("backend", "", None);
        invalid.ports = Some(vec!["8080:0".to_string()]);
        let result = validate_compile(&CompileSpec {
            services: vec![invalid],
            kubernetes: None,
            cicd: None,
        });
        let errors = result.expect_err("invalid service should fail");
        assert!(
            errors
                .iter()
                .any(|error| error.path == "services.backend.image")
        );
        assert!(
            errors
                .iter()
                .any(|error| error.path == "services.backend.ports[0]")
        );
    }

    #[test]
    fn reports_missing_and_circular_dependencies() {
        let result = validate_compile(&CompileSpec {
            services: vec![
                service(
                    "a",
                    "a:latest",
                    Some(vec!["missing".to_string(), "b".to_string()]),
                ),
                service("b", "b:latest", Some(vec!["a".to_string()])),
            ],
            kubernetes: None,
            cicd: None,
        });
        let errors = result.expect_err("invalid dependencies should fail");
        assert!(
            errors
                .iter()
                .any(|error| error.message.contains("does not exist"))
        );
        assert!(
            errors
                .iter()
                .any(|error| error.message.contains("circular dependency"))
        );
    }

    #[test]
    fn accepts_valid_services() {
        let result = validate_compile(&CompileSpec {
            services: vec![service("backend", "backend:latest", None)],
            kubernetes: None,
            cicd: None,
        });
        assert_eq!(result, Ok(()));
    }
}
