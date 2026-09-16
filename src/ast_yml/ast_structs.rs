use serde::{Deserialize, Serialize};
use serde_yaml::Number as YamlNumber;
use std::collections::BTreeMap;
use std::path::PathBuf;

// ============================================================
// compile.yaml
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileSpec {
    pub version: String,

    pub project: ProjectSpec,

    /// Какие файлы DevInit должен сгенерировать.
    ///
    /// Example:
    /// generate:
    ///   - compose
    ///   - kubernetes
    ///   - terraform
    ///   - github-actions
    #[serde(default)]
    pub generate: Vec<GenerateTarget>,

    #[serde(default)]
    pub variables: BTreeMap<String, VariableSpec>,

    #[serde(default)]
    pub services: BTreeMap<String, ServiceSpec>,

    #[serde(default)]
    pub volumes: BTreeMap<String, VolumeSpec>,

    #[serde(default)]
    pub networks: BTreeMap<String, NetworkSpec>,

    #[serde(default)]
    pub routes: BTreeMap<String, RouteSpec>,

    // Target-wide settings.
    pub compose: Option<ComposeConfigSpec>,
    pub kubernetes: Option<KubernetesConfigSpec>,
    pub terraform: Option<TerraformSpec>,

    pub github_actions: Option<GithubActionsSpec>,
    pub gitlab_ci: Option<GitLabCiSpec>,
}

// ============================================================
// Project
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSpec {
    pub name: String,

    pub description: Option<String>,

    /// Default: ./generated
    pub output: Option<PathBuf>,
}

// ============================================================
// Generation targets
// ============================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GenerateTarget {
    Compose,
    Kubernetes,
    Terraform,
    GithubActions,
    GitlabCi,
}

// ============================================================
// Variables
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableSpec {
    #[serde(rename = "type")]
    pub kind: VariableType,

    #[serde(default)]
    pub required: bool,

    pub default: Option<String>,

    pub description: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum VariableType {
    String,
    Integer,
    Boolean,
    Secret,
}

// ============================================================
// Service
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSpec {
    /// Recipe reference.
    ///
    /// Examples:
    /// use: postgres
    /// use: community/meilisearch
    /// use: acme/internal-postgres
    #[serde(rename = "use")]
    pub recipe: Option<String>,

    /// Software/service version.
    ///
    /// Example:
    /// version: "17"
    ///
    /// This is NOT the recipe version.
    pub version: Option<String>,

    /// Existing Dockerfile.
    ///
    /// DevInit v1 does NOT generate Dockerfiles.
    pub build: Option<DockerBuildSpec>,

    /// Existing image OR target image for a build.
    pub image: Option<ImageSpec>,

    #[serde(flatten)]
    pub runtime: ServiceRuntimeSpec,
}

// ============================================================
// Docker build
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerBuildSpec {
    pub context: PathBuf,

    /// Existing Dockerfile path.
    pub dockerfile: PathBuf,

    /// Optional multi-stage Docker target.
    pub target: Option<String>,

    #[serde(default)]
    pub args: BTreeMap<String, String>,
}

// ============================================================
// Image
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSpec {
    /// Examples:
    /// ghcr.io/flora/backend
    /// postgres
    pub repository: String,

    pub tag: Option<String>,

    pub pull_policy: Option<ImagePullPolicy>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ImagePullPolicy {
    Always,
    IfNotPresent,
    Never,
}

// ============================================================
// Runtime service configuration
// ============================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServiceRuntimeSpec {
    #[serde(default)]
    pub ports: Vec<PortSpec>,

    #[serde(default)]
    pub environment: BTreeMap<String, String>,

    #[serde(default)]
    pub volumes: Vec<VolumeMountSpec>,

    #[serde(default)]
    pub networks: Vec<String>,

    #[serde(default)]
    pub depends_on: Vec<String>,

    pub healthcheck: Option<HealthcheckSpec>,

    pub resources: Option<ResourceSpec>,

    pub deployment: Option<DeploymentSpec>,

    pub storage: Option<StorageSpec>,

    pub security: Option<SecuritySpec>,

    pub overrides: Option<TargetOverridesSpec>,
}

// ============================================================
// Ports
// ============================================================

/// Supports both:
///
/// ports:
///   - 8080
///
/// and:
///
/// ports:
///   - name: http
///     container: 8080
///     host: 8080
///     protocol: tcp
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortSpec {
    Simple(u16),
    Detailed(PortConfigSpec),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortConfigSpec {
    pub name: Option<String>,

    pub container: u16,

    pub host: Option<u16>,

    #[serde(default)]
    pub protocol: Protocol,
}

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    #[default]
    Tcp,
    Udp,
}

// ============================================================
// Volumes
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMountSpec {
    /// Top-level volume name or host path.
    pub source: String,

    /// Path inside the container.
    pub target: PathBuf,

    #[serde(default)]
    pub read_only: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VolumeSpec {
    pub driver: Option<String>,

    #[serde(default)]
    pub external: bool,

    #[serde(default)]
    pub labels: BTreeMap<String, String>,
}

// ============================================================
// Networks
// ============================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkSpec {
    pub driver: Option<String>,

    #[serde(default)]
    pub external: bool,

    #[serde(default)]
    pub labels: BTreeMap<String, String>,
}

// ============================================================
// Healthcheck
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthcheckSpec {
    /// Exactly one of:
    /// http / tcp / command
    pub http: Option<HttpHealthcheckSpec>,
    pub tcp: Option<TcpHealthcheckSpec>,
    pub command: Option<Vec<String>>,

    /// Examples: "10s", "1m"
    pub interval: Option<String>,

    pub timeout: Option<String>,

    pub retries: Option<u32>,

    pub start_period: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpHealthcheckSpec {
    pub path: String,

    pub port: PortRef,

    pub scheme: Option<HttpScheme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpHealthcheckSpec {
    pub port: PortRef,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HttpScheme {
    Http,
    Https,
}

// ============================================================
// Port references
// ============================================================

/// Supports:
///
/// port: 8080
///
/// or:
///
/// port: http
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortRef {
    Number(u16),
    Name(String),
}

// ============================================================
// Resources
// ============================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceSpec {
    pub requests: Option<ResourceValuesSpec>,
    pub limits: Option<ResourceValuesSpec>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceValuesSpec {
    /// Examples:
    /// "250m"
    /// "1"
    pub cpu: Option<String>,

    /// Examples:
    /// "256Mi"
    /// "1Gi"
    pub memory: Option<String>,
}

// ============================================================
// Deployment
// ============================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeploymentSpec {
    /// Default is determined by DevInit/recipe.
    pub workload: Option<WorkloadKind>,

    pub replicas: Option<u32>,

    pub strategy: Option<DeploymentStrategySpec>,

    pub autoscaling: Option<AutoscalingSpec>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum WorkloadKind {
    Deployment,
    StatefulSet,
    DaemonSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum DeploymentStrategySpec {
    Rolling {
        /// Examples:
        /// "1"
        /// "25%"
        pub max_surge: Option<String>,

        /// Examples:
        /// "0"
        /// "25%"
        pub max_unavailable: Option<String>,
    },

    Recreate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoscalingSpec {
    pub min: u32,
    pub max: u32,

    pub cpu_target: Option<u8>,
    pub memory_target: Option<u8>,
}

// ============================================================
// Persistent storage
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSpec {
    /// Examples:
    /// "10Gi"
    /// "500Gi"
    pub size: String,

    pub class: Option<String>,

    pub access_mode: Option<StorageAccessMode>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum StorageAccessMode {
    ReadWriteOnce,
    ReadOnlyMany,
    ReadWriteMany,
}

// ============================================================
// Security
// ============================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecuritySpec {
    pub run_as_non_root: Option<bool>,

    pub read_only_root_filesystem: Option<bool>,

    pub user: Option<u32>,

    pub group: Option<u32>,

    #[serde(default)]
    pub add_capabilities: Vec<String>,

    #[serde(default)]
    pub drop_capabilities: Vec<String>,
}

// ============================================================
// Routes
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteSpec {
    pub service: String,

    pub port: PortRef,

    pub host: Option<String>,

    /// Default should be "/".
    pub path: Option<String>,

    #[serde(default)]
    pub tls: bool,
}

// ============================================================
// Target overrides
// ============================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TargetOverridesSpec {
    pub compose: Option<ComposeServiceOverrideSpec>,

    pub kubernetes: Option<KubernetesServiceOverrideSpec>,
}

// ============================================================
// Docker Compose
// ============================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComposeConfigSpec {
    pub project_name: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComposeServiceOverrideSpec {
    pub container_name: Option<String>,

    pub restart: Option<RestartPolicy>,

    pub init: Option<bool>,

    #[serde(default)]
    pub extra_hosts: Vec<String>,

    #[serde(default)]
    pub profiles: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RestartPolicy {
    No,
    Always,
    OnFailure,
    UnlessStopped,
}

// ============================================================
// Kubernetes
// ============================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KubernetesConfigSpec {
    pub namespace: Option<String>,

    #[serde(default)]
    pub labels: BTreeMap<String, String>,

    #[serde(default)]
    pub annotations: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KubernetesServiceOverrideSpec {
    pub service_type: Option<KubernetesServiceType>,

    pub service_account: Option<String>,

    pub termination_grace_period_seconds: Option<u64>,

    #[serde(default)]
    pub labels: BTreeMap<String, String>,

    #[serde(default)]
    pub annotations: BTreeMap<String, String>,

    #[serde(default)]
    pub node_selector: BTreeMap<String, String>,

    #[serde(default)]
    pub tolerations: Vec<KubernetesTolerationSpec>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum KubernetesServiceType {
    ClusterIP,
    NodePort,
    LoadBalancer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesTolerationSpec {
    pub key: Option<String>,

    pub operator: Option<String>,

    pub value: Option<String>,

    pub effect: Option<String>,

    pub toleration_seconds: Option<u64>,
}

// ============================================================
// Terraform
// ============================================================

/// Terraform is intentionally target-specific.
///
/// DevInit does not attempt to hide all Terraform concepts.
/// Beginners can omit this section entirely.
/// DevOps users can use explicit Terraform resources/modules.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TerraformSpec {
    pub required_version: Option<String>,

    pub backend: Option<TerraformBackendSpec>,

    #[serde(default)]
    pub providers: BTreeMap<String, TerraformProviderSpec>,

    #[serde(default)]
    pub resources: BTreeMap<String, TerraformResourceSpec>,

    #[serde(default)]
    pub modules: BTreeMap<String, TerraformModuleSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformBackendSpec {
    /// Examples:
    /// local
    /// s3
    /// azurerm
    /// gcs
    #[serde(rename = "type")]
    pub backend_type: String,

    #[serde(default)]
    pub config: BTreeMap<String, TerraformValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformProviderSpec {
    /// Example:
    /// hashicorp/aws
    pub source: String,

    pub version: Option<String>,

    #[serde(default)]
    pub config: BTreeMap<String, TerraformValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformResourceSpec {
    /// Example:
    /// aws_s3_bucket
    #[serde(rename = "type")]
    pub resource_type: String,

    /// Optional provider alias/reference.
    pub provider: Option<String>,

    #[serde(default)]
    pub config: BTreeMap<String, TerraformValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerraformModuleSpec {
    pub source: String,

    pub version: Option<String>,

    #[serde(default)]
    pub inputs: BTreeMap<String, TerraformValue>,
}

/// Allows literals, nested structures and explicit HCL expressions.
///
/// Example expression:
///
/// region:
///   expr: var.aws_region
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TerraformValue {
    Expression {
        expr: String,
    },

    Bool(bool),

    Number(YamlNumber),

    String(String),

    List(Vec<TerraformValue>),

    Map(BTreeMap<String, TerraformValue>),

    Null,
}

// ============================================================
// GitHub Actions
// ============================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GithubActionsSpec {
    pub workflow_name: Option<String>,

    #[serde(default)]
    pub branches: Vec<String>,

    pub registry: Option<ContainerRegistrySpec>,
}

// ============================================================
// GitLab CI
// ============================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GitLabCiSpec {
    #[serde(default)]
    pub branches: Vec<String>,

    pub registry: Option<ContainerRegistrySpec>,
}

// ============================================================
// Container registry
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerRegistrySpec {
    /// Examples:
    /// ghcr.io
    /// registry.gitlab.com
    /// registry.company.kz
    pub host: String,

    /// Names of CI secrets/environment variables.
    pub username_secret: Option<String>,

    pub password_secret: Option<String>,
}

// ============================================================
// Recipe format
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeSpec {
    pub api_version: String,

    pub kind: RecipeKind,

    pub metadata: RecipeMetadataSpec,

    /// Container image used by the recipe.
    pub image: RecipeImageSpec,

    /// Supported/default software versions.
    pub versions: Option<RecipeVersionsSpec>,

    /// Environment variable schema.
    #[serde(default)]
    pub environment: BTreeMap<String, RecipeEnvironmentSpec>,

    /// Runtime defaults injected before user overrides.
    #[serde(default)]
    pub defaults: ServiceRuntimeSpec,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RecipeKind {
    Service,

    /// Reserved for future versions.
    Infrastructure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeMetadataSpec {
    /// Example:
    /// postgres
    pub name: String,

    /// Examples:
    /// devinit
    /// community
    /// acme
    pub namespace: String,

    /// Version of the recipe itself.
    ///
    /// NOT PostgreSQL/Redis/etc. version.
    pub version: String,

    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeImageSpec {
    /// Example:
    /// postgres
    pub repository: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeVersionsSpec {
    pub default: String,

    #[serde(default)]
    pub available: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecipeEnvironmentSpec {
    pub description: Option<String>,

    pub default: Option<String>,

    #[serde(default)]
    pub required: bool,

    #[serde(default)]
    pub secret: bool,
}

// ============================================================
// devinit.lock
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevInitLock {
    pub version: String,

    #[serde(default)]
    pub recipes: BTreeMap<String, LockedRecipeSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedRecipeSpec {
    /// Recipe version.
    pub version: String,

    /// Example:
    /// sha256:...
    pub checksum: String,
}