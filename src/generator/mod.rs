pub mod cicd_generator;
pub mod k8s_generator;
pub mod writer;

use crate::errors::GenerationError;
use crate::models::compile_struct::CompileSpec;
use std::path::{Path, PathBuf};

pub fn generate(spec: &CompileSpec, output_dir: &Path) -> Result<Vec<PathBuf>, GenerationError> {
    let mut generated = Vec::new();

    if let Some(kubernetes) = &spec.kubernetes {
        generated.extend(k8s_generator::generate_k8s(kubernetes, output_dir)?);
    }
    if let Some(cicd) = &spec.cicd {
        generated.extend(cicd_generator::generate_cicd(cicd, output_dir)?);
    }

    Ok(generated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::cicd_struct::{CiProvider, Pipeline};
    use crate::models::k8s_struct::{
        Container, Deployment, DeploymentSpec, LabelSelector, ObjectMeta, PodSpec, PodTemplateSpec,
    };
    use std::collections::HashMap;

    fn deployment() -> Deployment {
        Deployment {
            api_version: "apps/v1".to_string(),
            kind: "Deployment".to_string(),
            metadata: ObjectMeta {
                name: "backend".to_string(),
                namespace: None,
                labels: None,
                annotations: None,
            },
            spec: DeploymentSpec {
                replicas: Some(1),
                selector: LabelSelector {
                    match_labels: HashMap::new(),
                },
                template: PodTemplateSpec {
                    metadata: ObjectMeta {
                        name: "backend".to_string(),
                        namespace: None,
                        labels: None,
                        annotations: None,
                    },
                    spec: PodSpec {
                        containers: vec![Container {
                            name: "backend".to_string(),
                            image: "backend:latest".to_string(),
                            image_pull_policy: None,
                            command: None,
                            args: None,
                            working_dir: None,
                            ports: None,
                            env: None,
                            env_from: None,
                            resources: None,
                            volume_mounts: None,
                            liveness_probe: None,
                            readiness_probe: None,
                            startup_probe: None,
                            security_context: None,
                        }],
                        init_containers: None,
                        volumes: None,
                        restart_policy: None,
                        termination_grace_period_seconds: None,
                        service_account_name: None,
                        image_pull_secrets: None,
                        node_selector: None,
                        security_context: None,
                    },
                },
                strategy: None,
                min_ready_seconds: None,
                revision_history_limit: None,
                progress_deadline_seconds: None,
                paused: None,
            },
        }
    }

    #[test]
    fn generates_all_configured_targets() {
        let output =
            std::env::temp_dir().join(format!("devinit-generator-test-{}", std::process::id()));
        let spec = CompileSpec {
            services: Vec::new(),
            kubernetes: Some(deployment()),
            cicd: Some(Pipeline {
                provider: CiProvider::Github,
                triggers: Vec::new(),
                jobs: Vec::new(),
            }),
        };

        let paths = generate(&spec, &output).expect("generation should succeed");
        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&output.join("k8s/backend-deployment.yaml")));
        assert!(paths.contains(&output.join(".github/workflows/devinit.yml")));
        let _ = std::fs::remove_dir_all(output);
    }
}
