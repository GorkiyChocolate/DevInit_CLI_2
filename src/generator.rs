pub mod cicd_generator;
pub mod docker_compose_generator;
pub mod k8s_generator;

use crate::errors::GenerationError;
use crate::models::compile_struct::CompileSpec;
use std::path::{Path, PathBuf};

pub fn generate(spec: &CompileSpec, output_dir: &Path) -> Result<Vec<PathBuf>, GenerationError> {
    let mut generated = Vec::new();

    if !spec.services.is_empty() {
        generated.extend(docker_compose_generator::generate_docker_compose(
            &spec.services,
            output_dir,
        )?);
    }

    if let Some(kubernetes) = &spec.kubernetes {
        generated.extend(k8s_generator::generate_k8s(kubernetes, output_dir)?);
    }
    if let Some(cicd) = &spec.cicd {
        generated.extend(cicd_generator::generate_cicd(cicd, output_dir)?);
    }

    Ok(generated)
}
