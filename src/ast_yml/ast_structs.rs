use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

/// Корневое промежуточное представление (IR)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct CompileSpec {
    pub project: ProjectMeta,
    #[serde(default)]
    pub variables: BTreeMap<String, VariableDeclaration>,
    #[serde(default)]
    pub networks: BTreeMap<String, NetworkSpec>,
    #[serde(default)]
    pub volumes: BTreeMap<String, VolumeSpec>,
    #[serde(default)]
    pub services: BTreeMap<String, ServiceSpec>,
    #[serde(default)]
    pub pipeline: Option<PipelineSpec>,
    #[serde(default)]
    pub environments: BTreeMap<String, EnvironmentSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ProjectMeta {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct VariableDeclaration {
    #[serde(rename = "type")]
    pub var_type: VariableType,
    #[serde(default)]
    pub required: bool,
    pub default: Option<serde_yaml::Value>,
    pub description: Option<String>,
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
    #[serde(rename = "type")]
    pub network_type: NetworkType,
    #[serde(default)]
    pub internal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NetworkType {
    Bridge,
    Overlay,
    Host,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct VolumeSpec {
    #[serde(rename = "type")]
    pub volume_type: VolumeType,
    pub size: Option<String>, // "10Gi", "512Mi" (для K8s PVC / Cloud Storage)
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
    #[serde(rename = "type")]
    pub service_type: ServiceRole,
    pub version: Option<String>,
    pub runtime: Option<RuntimeSpec>,
    pub image: Option<ImageSpec>,
    pub build: Option<BuildSpec>,
    #[serde(default)]
    pub ports: Vec<PortSpec>,
    #[serde(default)]
    pub environment: BTreeMap<String, EnvVarValue>,
    #[serde(default)]
    pub storage: Vec<StorageMount>,
    #[serde(default)]
    pub routing: Option<RoutingSpec>,
    #[serde(default)]
    pub depends_on: BTreeMap<String, DependencyCondition>,
    #[serde(default)]
    pub networks: Vec<String>,
    pub healthcheck: Option<HealthcheckSpec>,
    pub resources: Option<ResourceLimitsSpec>,
    pub replicas: Option<ReplicasSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ServiceRole {
    Web,
    Api,
    Worker,
    Database,
    Cache,
    Broker,
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
    #[serde(default = "default_dockerfile")]
    pub dockerfile: String,
    pub target: Option<String>,
    #[serde(default)]
    pub args: BTreeMap<String, String>,
}

fn default_dockerfile() -> String {
    "Dockerfile".to_string()
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

/// Сетевая маршрутизация (Ingress / Reverse Proxy / Load Balancer)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct RoutingSpec {
    pub host: Option<String>,
    #[serde(default)]
    pub paths: Vec<PathRoute>,
    #[serde(default)]
    pub tls: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PathRoute {
    pub path: String,
    pub target_port: u16,
    #[serde(default = "default_strip_prefix")]
    pub strip_prefix: bool,
}

fn default_strip_prefix() -> bool {
    false
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EnvVarValue {
    Ref(VariableRef),
    SecretRef(SecretRef),
    Literal(serde_yaml::Value),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct VariableRef {
    pub variable: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct SecretRef {
    pub secret: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct StorageMount {
    pub volume: String,
    pub mount: String,
    #[serde(default)]
    pub read_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DependencyCondition {
    Started,
    Healthy,
    CompletedSuccessfully,
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

/// Метаданные CI/CD пайплайнов (генерация GitHub Actions, GitLab CI)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PipelineSpec {
    #[serde(default)]
    pub stages: Vec<PipelineStage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PipelineStage {
    pub name: String,
    #[serde(default)]
    pub steps: Vec<PipelineStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct PipelineStep {
    pub name: String,
    pub command: Option<String>,
    pub run_in_service: Option<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

/// Спецификация окружений (dev, staging, prod)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct EnvironmentSpec {
    #[serde(default)]
    pub variables: BTreeMap<String, serde_yaml::Value>,
    #[serde(default)]
    pub services: BTreeMap<String, ServiceOverrideSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ServiceOverrideSpec {
    pub replicas: Option<ReplicasSpec>,
    pub resources: Option<ResourceLimitsSpec>,
    #[serde(default)]
    pub environment: BTreeMap<String, serde_yaml::Value>,
}