use crate::errors::GenerationError;
use crate::generator::writer::write_yaml;
use crate::models::cicd_struct::{CiProvider, Pipeline, Runner, StepAction, Trigger};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub fn generate_cicd(
    pipeline: &Pipeline,
    output_dir: &Path,
) -> Result<Vec<PathBuf>, GenerationError> {
    match pipeline.provider {
        CiProvider::Github => generate_github_actions(pipeline, output_dir),
        CiProvider::Gitlab => generate_gitlab_ci(pipeline, output_dir),
    }
}

fn generate_github_actions(
    pipeline: &Pipeline,
    output_dir: &Path,
) -> Result<Vec<PathBuf>, GenerationError> {
    let workflow = GithubWorkflow::from_pipeline(pipeline);
    let path = output_dir.join(".github/workflows/devinit.yml");
    Ok(vec![write_yaml(&workflow, &path)?])
}

fn generate_gitlab_ci(
    pipeline: &Pipeline,
    output_dir: &Path,
) -> Result<Vec<PathBuf>, GenerationError> {
    let workflow = GitlabPipeline::from_pipeline(pipeline);
    let path = output_dir.join(".gitlab-ci.yml");
    Ok(vec![write_yaml(&workflow, &path)?])
}

#[derive(Serialize)]
struct GithubWorkflow {
    name: &'static str,
    #[serde(rename = "on")]
    triggers: BTreeMap<String, GithubTrigger>,
    jobs: BTreeMap<String, GithubJob>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum GithubTrigger {
    Branches { branches: Vec<String> },
    Empty(BTreeMap<String, String>),
}

#[derive(Serialize)]
struct GithubJob {
    #[serde(rename = "runs-on")]
    runs_on: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    needs: Vec<String>,
    steps: Vec<GithubStep>,
}

#[derive(Serialize)]
struct GithubStep {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    uses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run: Option<String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    env: BTreeMap<String, String>,
}

impl GithubWorkflow {
    fn from_pipeline(pipeline: &Pipeline) -> Self {
        let mut triggers = BTreeMap::new();
        for trigger in &pipeline.triggers {
            match trigger {
                Trigger::Push { branches } => {
                    triggers.insert("push".to_string(), github_trigger(branches));
                }
                Trigger::PullRequest { branches } => {
                    triggers.insert("pull_request".to_string(), github_trigger(branches));
                }
                Trigger::Manual => {
                    triggers.insert(
                        "workflow_dispatch".to_string(),
                        GithubTrigger::Empty(BTreeMap::new()),
                    );
                }
            }
        }

        let jobs = pipeline
            .jobs
            .iter()
            .map(|job| {
                (
                    job.name.clone(),
                    GithubJob {
                        runs_on: runner_name(&job.runner).to_string(),
                        needs: job.needs.clone(),
                        steps: job.steps.iter().map(github_step).collect(),
                    },
                )
            })
            .collect();

        Self {
            name: "DevInit",
            triggers,
            jobs,
        }
    }
}

fn github_trigger(branches: &[String]) -> GithubTrigger {
    if branches.is_empty() {
        GithubTrigger::Empty(BTreeMap::new())
    } else {
        GithubTrigger::Branches {
            branches: branches.to_vec(),
        }
    }
}

fn runner_name(runner: &Runner) -> &'static str {
    match runner {
        Runner::UbuntuLatest => "ubuntu-latest",
        Runner::WindowsLatest => "windows-latest",
        Runner::MacosLatest => "macos-latest",
    }
}

fn github_step(step: &crate::models::cicd_struct::Step) -> GithubStep {
    let (uses, run) = match &step.action {
        StepAction::Checkout => (Some("actions/checkout@v4".to_string()), None),
        StepAction::Run { command } => (None, Some(command.clone())),
        StepAction::DockerBuild { dockerfile, image } => (
            None,
            Some(format!("docker build -f {dockerfile} -t {image} .")),
        ),
        StepAction::DockerPush { image } => (None, Some(format!("docker push {image}"))),
    };
    GithubStep {
        name: step.name.clone(),
        uses,
        run,
        env: step.env.clone(),
    }
}

#[derive(Serialize)]
struct GitlabPipeline {
    stages: Vec<String>,
    #[serde(flatten)]
    jobs: BTreeMap<String, GitlabJob>,
}

#[derive(Serialize)]
struct GitlabJob {
    stage: String,
    script: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    needs: Vec<String>,
}

impl GitlabPipeline {
    fn from_pipeline(pipeline: &Pipeline) -> Self {
        let stages = pipeline.jobs.iter().map(|job| job.name.clone()).collect();
        let jobs = pipeline
            .jobs
            .iter()
            .map(|job| {
                (
                    job.name.clone(),
                    GitlabJob {
                        stage: job.name.clone(),
                        script: job.steps.iter().filter_map(gitlab_command).collect(),
                        needs: job.needs.clone(),
                    },
                )
            })
            .collect();
        Self { stages, jobs }
    }
}

fn gitlab_command(step: &crate::models::cicd_struct::Step) -> Option<String> {
    match &step.action {
        StepAction::Checkout => None,
        StepAction::Run { command } => Some(command.clone()),
        StepAction::DockerBuild { dockerfile, image } => {
            Some(format!("docker build -f {dockerfile} -t {image} ."))
        }
        StepAction::DockerPush { image } => Some(format!("docker push {image}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::cicd_struct::{Job, Runner, Step, StepAction};
    use std::collections::BTreeMap;

    #[test]
    fn generates_github_workflow_path_and_yaml() {
        let output = std::env::temp_dir().join(format!("devinit-cicd-test-{}", std::process::id()));
        let pipeline = Pipeline {
            provider: CiProvider::Github,
            triggers: Vec::new(),
            jobs: vec![Job {
                name: "build".to_string(),
                runner: Runner::UbuntuLatest,
                needs: Vec::new(),
                steps: vec![Step {
                    name: "Build".to_string(),
                    action: StepAction::Run {
                        command: "cargo build".to_string(),
                    },
                    env: BTreeMap::new(),
                }],
            }],
        };
        let paths = generate_cicd(&pipeline, &output).expect("generation should succeed");
        assert_eq!(paths, vec![output.join(".github/workflows/devinit.yml")]);
        let contents = std::fs::read_to_string(&paths[0]).expect("generated file should exist");
        assert!(!contents.contains("provider: github"));
        assert!(contents.contains("jobs:"));
        assert!(contents.contains("runs-on: ubuntu-latest"));
        assert!(contents.contains("steps:"));
        let _ = std::fs::remove_dir_all(output);
    }

    #[test]
    fn generates_gitlab_pipeline_path_and_structure() {
        let output =
            std::env::temp_dir().join(format!("devinit-gitlab-test-{}", std::process::id()));
        let pipeline = Pipeline {
            provider: CiProvider::Gitlab,
            triggers: Vec::new(),
            jobs: vec![Job {
                name: "build".to_string(),
                runner: Runner::UbuntuLatest,
                needs: Vec::new(),
                steps: vec![Step {
                    name: "Build".to_string(),
                    action: StepAction::Run {
                        command: "cargo build".to_string(),
                    },
                    env: BTreeMap::new(),
                }],
            }],
        };
        let paths = generate_cicd(&pipeline, &output).expect("generation should succeed");
        assert_eq!(paths, vec![output.join(".gitlab-ci.yml")]);
        let contents = std::fs::read_to_string(&paths[0]).expect("generated file should exist");
        assert!(contents.contains("stages:"));
        assert!(contents.contains("build:"));
        assert!(contents.contains("script:"));
        let _ = std::fs::remove_dir_all(output);
    }
}
