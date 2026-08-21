use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::structs::{ConfigsList, RecipeCompose};

pub fn yaml_data(config_struct: &RecipeCompose, path: &PathBuf) -> std::io::Result<()> {
    append_recipes(std::slice::from_ref(config_struct), path)
}

pub fn yaml_configs_data(configs_list: &ConfigsList, path: &PathBuf) -> std::io::Result<()> {
    append_recipes(&configs_list.configs, path)
}

fn append_recipes(recipes: &[RecipeCompose], path: &PathBuf) -> std::io::Result<()> {
    let existing_names = read_existing_names(path)?;
    let mut names_to_add = HashMap::new();

    for recipe in recipes {
        if existing_names.contains_key(&recipe.name) || names_to_add.contains_key(&recipe.name) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("Config '{}' already exists in YAML file", recipe.name),
            ));
        }
        names_to_add.insert(recipe.name.clone(), ());
    }

    if recipes.is_empty() {
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    if path.metadata()?.len() > 0 {
        file.write_all(b"\n")?;
    }

    for recipe in recipes {
        let mut recipe_map = HashMap::new();
        recipe_map.insert(&recipe.name, recipe);
        let yaml = serde_yaml::to_string(&recipe_map)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        file.write_all(yaml.as_bytes())?;
    }

    Ok(())
}

fn read_existing_names(path: &PathBuf) -> std::io::Result<HashMap<String, RecipeCompose>> {
    if !path.exists() {
        return Ok(HashMap::new());
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.trim().is_empty() {
        return Ok(HashMap::new());
    }

    serde_yaml::from_str(&contents)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}
