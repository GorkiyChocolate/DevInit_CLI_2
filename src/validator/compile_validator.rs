use crate::errors::ValidationError;
use crate::models::compile_struct::CompileSpec;
use std::collections::{HashMap, HashSet};

pub fn validate_compile(spec: &CompileSpec) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();
    for service in &spec.services {
        let recipe = service.to_recipe();
        crate::validator::services_validator::validate_service(&recipe.name, &recipe, &mut errors);
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
        let recipe = service.to_recipe();
        if services.insert(recipe.name.clone(), index).is_some() {
            errors.push(ValidationError {
                path: format!("services[{index}].name"),
                message: format!("service '{}' is defined more than once", recipe.name),
            });
        }
    }
    for (index, service) in spec.services.iter().enumerate() {
        let recipe = service.to_recipe();
        if let Some(dependencies) = &recipe.depends_on {
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
    services: &HashMap<String, usize>,
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
    services: &HashMap<String, usize>,
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
    let recipe = spec.services[index].to_recipe();
    let cycle = recipe.depends_on.as_ref().is_some_and(|dependencies| {
        dependencies
            .iter()
            .filter_map(|name| services.get(name))
            .any(|dependency| visit_service(*dependency, spec, services, visiting, visited))
    });
    visiting.remove(&index);
    cycle
}
