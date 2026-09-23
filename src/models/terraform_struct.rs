use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct TerraformConfig {
    pub terraform: TerraformBlock,
    pub providers: BTreeMap<String, Provider>,
    pub variables: BTreeMap<String, Variable>,
    pub resources: Vec<Resource>,
    pub outputs: BTreeMap<String, Output>,
}

#[derive(Debug, Clone)]
pub struct TerraformBlock {
    pub required_version: Option<String>,
    pub required_providers: BTreeMap<String, RequiredProvider>,
}

#[derive(Debug, Clone)]
pub struct RequiredProvider {
    pub source: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Provider {
    pub name: String,
    pub attributes: BTreeMap<String, TerraformValue>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub variable_type: VariableType,
    pub description: Option<String>,
    pub default: Option<TerraformValue>,
    pub sensitive: bool,
    pub nullable: bool,
}

#[derive(Debug, Clone)]
pub enum VariableType {
    String,
    Number,
    Bool,

    List(Box<VariableType>),
    Set(Box<VariableType>),
    Map(Box<VariableType>),

    Object(BTreeMap<String, VariableType>),
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub resource_type: String,
    pub name: String,
    pub attributes: BTreeMap<String, TerraformValue>,
    pub blocks: BTreeMap<String, Vec<Block>>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub attributes: BTreeMap<String, TerraformValue>,
}

#[derive(Debug, Clone)]
pub struct Output {
    pub value: TerraformValue,
    pub description: Option<String>,
    pub sensitive: bool,
}

#[derive(Debug, Clone)]
pub enum TerraformValue {
    String(String),
    Number(f64),
    Bool(bool),

    List(Vec<TerraformValue>),
    Map(BTreeMap<String, TerraformValue>),

    // Например:
    // var.region
    // aws_instance.backend.id
    // local.project_name
    Expression(String),

    Null,
}