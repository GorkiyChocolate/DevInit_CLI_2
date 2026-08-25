use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct CompileSpec {
    pub project: ProjectMeta,
    #[serde(default)]
    pub variables: Vec<VariableDeclaration>,
    #[serde(default)]
    pub networks: Vec<NetworkSpec>,
    #[serde(default)]
    pub volumes: Vec<VolumeSpec>,
    #[serde(default)]
    pub services: Vec<ServiceSpec>,
    #[serde(default)]
    pub environments: Vec<EnvironmentSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ProjectMeta {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct VariableDeclaration {
    pub name: String,
    #[serde(rename = "type")]
    pub var_type: VariableType,
    #[serde(default)]
    pub required: bool,
    pub default: Option<serde_yaml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VariableType {
    String,
    Integer,
    Boolean,
    Secret,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct NetworkSpec {
    pub name: String,
    #[serde(rename = "type")]
    pub network_type: NetworkType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NetworkType {
    Public,
    Private,
    Overlay,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct VolumeSpec {
    pub name: String,
    #[serde(rename = "type")]
    pub volume_type: VolumeType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VolumeType {
    Persistent,
    Tmpfs,
    Bind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ServiceSpec {
    pub name: String,
    #[serde(rename = "type")]
    pub service_type: String,
    pub version: Option<String>,
    pub runtime: Option<RuntimeSpec>,
    pub image: Option<ImageSpec>,
    pub build: Option<BuildSpec>,
    #[serde(default)]
    pub ports: Vec<PortSpec>,
    #[serde(default)]
    pub environment: Vec<EnvironmentVariableAssignment>,
    #[serde(default)]
    pub storage: Vec<StorageMount>,
    #[serde(default)]
    pub depends_on: Vec<DependencySpec>,
    #[serde(default)]
    pub networks: Vec<String>,
    pub healthcheck: Option<HealthcheckSpec>,
    pub resources: Option<ResourceLimitsSpec>,
    pub replicas: Option<ReplicasSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct RuntimeSpec {
    pub language: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ImageSpec {
    pub name: String,
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct BuildSpec {
    pub context: String,
    pub dockerfile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PortSpec {
    pub container: u16,
    pub host: Option<u16>,
    #[serde(default = "default_protocol")]
    pub protocol: String,
}

fn default_protocol() -> String {
    "tcp".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct EnvironmentVariableAssignment {
    pub name: String,
    pub value: EnvVarValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EnvVarValue {
    Ref(VariableRef),
    Literal(serde_yaml::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct VariableRef {
    pub variable: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct StorageMount {
    pub volume: String,
    pub mount: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct DependencySpec {
    pub service: String,
    #[serde(default = "default_condition")]
    pub condition: String, // "ready", "started", "healthy"
}

fn default_condition() -> String {
    "started".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct HealthcheckSpec {
    pub command: Vec<String>,
    pub interval: String,
    pub timeout: String,
    pub retries: u32,
    pub start_period: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ResourceLimitsSpec {
    pub cpu: Option<ResourceBoundary>,
    pub memory: Option<ResourceBoundary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ResourceBoundary {
    pub request: Option<String>,
    pub limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ReplicasSpec {
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct EnvironmentSpec {
    pub name: String,
    #[serde(default)]
    pub variables: Vec<EnvironmentVariableOverride>,
    #[serde(default)]
    pub services: BTreeMap<String, ServiceOverrideSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct EnvironmentVariableOverride {
    pub name: String,
    pub value: serde_yaml::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ServiceOverrideSpec {
    pub replicas: Option<ReplicasSpec>,
    pub resources: Option<ResourceLimitsSpec>,
}