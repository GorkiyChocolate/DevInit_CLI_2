use crate::errors::DevinitError;
use crate::file_config::file_validator::{ensure_file_not_empty, ensure_images_are_new};
use crate::models::compile_struct::CompileSpec;
use crate::models::docker_compose_struct::{ConfigsList, RecipeCompose};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn load_compile(path: &Path) -> Result<CompileSpec, DevinitError> {
    if !path.exists() {
        return Err(DevinitError::ConfigurationError(format!(
            "compile file '{}' was not found",
            path.display()
        )));
    }
    ensure_file_not_empty(path)?;
    let contents =
        std::fs::read_to_string(path).map_err(|error| DevinitError::FileIOError(error))?;
    Ok(serde_yaml::from_str(&contents)?)
}

pub fn yaml_data(config_struct: &RecipeCompose, path: &PathBuf) -> std::io::Result<()> {
    append_recipes(std::slice::from_ref(config_struct), path)
}

pub fn yaml_configs_data(configs_list: &ConfigsList, path: &PathBuf) -> std::io::Result<()> {
    append_recipes(&configs_list.configs, path)
}

fn append_recipes(recipes: &[RecipeCompose], path: &PathBuf) -> std::io::Result<()> {
    if recipes.is_empty() {
        return Ok(());
    }
    ensure_images_are_new(path, recipes)?;

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    if path.metadata()?.len() > 0 {
        file.write_all(b"\n")?;
    }

    for recipe in recipes {
        let mut recipe_map = HashMap::new();
        let mut compose_recipe = recipe.clone();
        compose_recipe.env = None;
        recipe_map.insert(&recipe.name, &compose_recipe);
        let yaml = serde_yaml::to_string(&recipe_map)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        file.write_all(yaml.as_bytes())?;
    }

    Ok(())
}
