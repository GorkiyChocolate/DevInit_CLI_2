use crate::ast_yml::ast_structs::CompileSpec;

fn validate_ast(spec: &CompileSpec)
    -> Result<(), Vec<ValidationError>> {

}

fn check_broken_reference(spec: &CompileSpec)
    -> Result<(), Vec<ValidationError>>{

}

fn detect_cyclic_dependencies(spec: &CompileSpec)
    -> Result<(), CyclicDependencyError> {

}