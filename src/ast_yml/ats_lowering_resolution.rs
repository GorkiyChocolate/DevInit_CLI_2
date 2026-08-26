use std::collections::BTreeMap;

use crate::ast_yml::ast_structs::{CompileSpec, EnvVarValue, VariableDeclaration};

fn resolve_for_environment(
    spec: &CompileSpec,
    env_name: Option<&str>)
    -> Result<ResolvedSpec, ResolveError>
    {

}

fn substitute_variables(
    env_vars: &BTreeMap<String, EnvVarValue>, 
    declared_vars: &BTreeMap<String, VariableDeclaration>, 
    overrides: &BTreeMap<String, String>)
    -> BTreeMap<String, String> {

} 