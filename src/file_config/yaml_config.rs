use crate::models::structs::{ConfigsList, RecipeCompose};
use std::collections::{HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn yaml_data(config_struct: &RecipeCompose, path: &PathBuf) -> std::io::Result<()> {
    append_recipes(std::slice::from_ref(config_struct), path)
}

pub fn yaml_configs_data(configs_list: &ConfigsList, path: &PathBuf) -> std::io::Result<()> {
    append_recipes(&configs_list.configs, path)
}

fn append_recipes(recipes: &[RecipeCompose], path: &PathBuf) -> std::io::Result<()> {
    let existing_images = read_existing_images(path)?;
    let mut images_to_add = HashSet::new();

    for recipe in recipes {
        if existing_images.contains(&recipe.image) || !images_to_add.insert(recipe.image.clone()) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("Image '{}' already exists in YAML file", recipe.image),
            ));
        }
    }

    if recipes.is_empty() {
        return Ok(());
    }

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

fn read_existing_images(path: &PathBuf) -> std::io::Result<HashSet<String>> {
    if !path.exists() {
        return Ok(HashSet::new());
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.trim().is_empty() {
        return Ok(HashSet::new());
    }

    let recipes: HashMap<String, RecipeCompose> = serde_yaml::from_str(&contents)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    Ok(recipes.into_values().map(|recipe| recipe.image).collect())
}
