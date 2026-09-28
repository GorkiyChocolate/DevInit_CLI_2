use crate::errors::{DevinitError, ValidationError};
use crate::{api, cli, file_config, generator, validator};
use std::{
    env,
    path::{Path, PathBuf},
};

use api::add_recipe::add_recipe;
use api::get_config::get_config;
use cli::commands;
use file_config::env_config::env_file_config;
use file_config::yaml_config::{load_compile, yaml_configs_data, yaml_data};

pub fn compile() -> Result<Vec<PathBuf>, DevinitError> {
    compile_at(Path::new("compile.yaml"), Path::new("."))
}

fn compile_at(input_path: &Path, output_dir: &Path) -> Result<Vec<PathBuf>, DevinitError> {
    let spec = load_compile(input_path)?;
    if let Err(errors) = validator::compile_validator::validate_compile(&spec) {
        return Err(DevinitError::ValidationErrors(errors));
    }
    Ok(generator::generate(&spec, output_dir)?)
}

fn print_validation_errors(errors: &[ValidationError]) {
    eprintln!("Configuration validation failed:");
    for error in errors {
        eprintln!("\n✗ {}\n  {}", error.path, error.message);
    }
}

pub async fn cli_logic() -> Result<(), DevinitError> {
    let base_url = "http://127.0.0.1:3000/services/";
    let configs_url = "http://127.0.0.1:3000/configs/";
    let matches = commands::build_cli().get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let recipe_name = sub_matches
                .get_one::<String>("service")
                .expect("Required argument");

            println!("Sending request for recipe: {}", recipe_name);
            match add_recipe(recipe_name, base_url).await {
                Ok(recipe) => {
                    println!("Successfully retrieved recipe: {:?}", recipe);
                    let target_path = env::current_dir()
                        .unwrap_or_else(|_| PathBuf::from("."))
                        .join("compose.yaml.example");
                    let env_path = env::current_dir()
                        .unwrap_or_else(|_| PathBuf::from("."))
                        .join("env.example");

                    if let Err(e) = yaml_data(&recipe, &target_path) {
                        eprintln!("Error updating {}: {}", target_path.display(), e);
                    }
                    if let Err(e) = env_file_config(&recipe, &env_path) {
                        eprintln!("Error updating {}: {}", env_path.display(), e);
                    }
                }
                Err(e) => eprintln!("Error fetching recipe: {}", e),
            }
        }

        Some(("list", sub_matches)) => {
            let service_type = sub_matches.get_one::<String>("type");
            let page = sub_matches.get_one::<String>("page");

            println!(
                "Fetching list... Type: {:?}, Page: {:?}",
                service_type, page
            );
        }

        Some(("get", sub_matches)) => {
            let config_name = sub_matches
                .get_one::<String>("service_url")
                .expect("Required argument");
            println!("Getting repository from: {}", configs_url);

            match get_config(configs_url, config_name).await {
                Ok(configs_list) => {
                    println!("Succesfully retrieved configs: {:?}", configs_list);
                    let target_path = env::current_dir()
                        .unwrap_or_else(|_| PathBuf::from("."))
                        .join("compose.yaml.example");

                    if let Err(e) = yaml_configs_data(&configs_list, &target_path) {
                        eprintln!("Error updating {}: {}", target_path.display(), e);
                    }
                }
                Err(e) => eprintln!("Error fetching config: {}", e),
            }
        }

        Some(("login", sub_matches)) => {
            let config_name = sub_matches
                .get_one::<String>("login_url")
                .expect("Required argument");
            println!("logging in: {}, {:?}", configs_url, config_name);
        }

        Some(("compile", _)) => match compile() {
            Ok(paths) => {
                println!("✓ compile.yaml loaded");
                println!("✓ configuration validated");
                println!("\nGenerated:");
                for path in &paths {
                    println!("  {}", path.display());
                }
                println!("\n✓ {} files generated", paths.len());
            }
            Err(DevinitError::ValidationErrors(errors)) => {
                print_validation_errors(&errors);
                return Err(DevinitError::ValidationErrors(errors));
            }
            Err(error) => {
                eprintln!("Compile failed: {error}");
                return Err(error);
            }
        },

        Some(("test", _)) => {
            println!("generation requires a validated CompileSpec");
        }
        _ => {
            println!("No subcommand provided. Use --help for usage instructions.");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_paths(name: &str) -> (PathBuf, PathBuf) {
        let root =
            std::env::temp_dir().join(format!("devinit-compile-{name}-{}", std::process::id()));
        (root.join("compile.yaml"), root.join("generated"))
    }

    fn valid_yaml() -> &'static str {
        "services: []\nkubernetes:\n  apiVersion: apps/v1\n  kind: Deployment\n  metadata:\n    name: backend\n  spec:\n    replicas: 1\n    selector:\n      matchLabels: {}\n    template:\n      metadata:\n        name: backend\n      spec:\n        containers:\n          - name: backend\n            image: backend:latest\ncicd: null\n"
    }

    #[test]
    fn valid_config_validates_then_generates() {
        let (input, output) = temp_paths("valid");
        fs::create_dir_all(input.parent().unwrap()).unwrap();
        fs::write(&input, valid_yaml()).unwrap();

        let paths = compile_at(&input, &output).expect("valid compile should succeed");
        assert_eq!(paths, vec![output.join("k8s/backend-deployment.yaml")]);
        assert!(paths[0].exists());
        let _ = fs::remove_dir_all(input.parent().unwrap());
    }

    #[test]
    fn invalid_config_does_not_generate_files() {
        let (input, output) = temp_paths("invalid");
        fs::create_dir_all(input.parent().unwrap()).unwrap();
        fs::write(
            &input,
            "services:\n  - name: backend\n    image: \"\"\nkubernetes: null\ncicd: null\n",
        )
        .unwrap();

        let result = compile_at(&input, &output);
        assert!(matches!(result, Err(DevinitError::ValidationErrors(_))));
        assert!(!output.exists());
        let _ = fs::remove_dir_all(input.parent().unwrap());
    }

    #[test]
    fn malformed_yaml_fails_before_validation_or_generation() {
        let (input, output) = temp_paths("malformed");
        fs::create_dir_all(input.parent().unwrap()).unwrap();
        fs::write(&input, "services: [").unwrap();

        let result = compile_at(&input, &output);
        assert!(matches!(result, Err(DevinitError::YamlParseError(_))));
        assert!(!output.exists());
        let _ = fs::remove_dir_all(input.parent().unwrap());
    }

    #[test]
    fn missing_compile_file_returns_an_error() {
        let (input, output) = temp_paths("missing");
        let result = compile_at(&input, &output);
        assert!(matches!(result, Err(DevinitError::ConfigurationError(_))));
        assert!(!output.exists());
    }

    #[test]
    fn repeated_compile_replaces_the_same_generated_file() {
        let (input, output) = temp_paths("repeat");
        fs::create_dir_all(input.parent().unwrap()).unwrap();
        fs::write(&input, valid_yaml()).unwrap();

        let first = compile_at(&input, &output).expect("first compile should succeed");
        let second = compile_at(&input, &output).expect("second compile should succeed");
        assert_eq!(first, second);
        assert_eq!(fs::read_dir(output.join("k8s")).unwrap().count(), 1);
        let _ = fs::remove_dir_all(input.parent().unwrap());
    }
}
