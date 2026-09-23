use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub provider: CiProvider,

    #[serde(default)]
    pub triggers: Vec<Trigger>,

    #[serde(default)]
    pub jobs: Vec<Job>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CiProvider {
    Github,
    Gitlab,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Trigger {
    Push {
        #[serde(default)]
        branches: Vec<String>,
    },

    PullRequest {
        #[serde(default)]
        branches: Vec<String>,
    },

    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub name: String,

    #[serde(default)]
    pub runner: Runner,

    #[serde(default)]
    pub needs: Vec<String>,

    #[serde(default)]
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Runner {
    #[default]
    UbuntuLatest,

    WindowsLatest,

    MacosLatest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub name: String,

    #[serde(flatten)]
    pub action: StepAction,

    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum StepAction {
    Checkout,

    Run {
        command: String,
    },

    DockerBuild {
        dockerfile: String,
        image: String,
    },

    DockerPush {
        image: String,
    },
}