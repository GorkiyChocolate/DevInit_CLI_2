use crate::models::docker_compose_struct::RecipeCompose;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

pub fn ensure_file_not_empty(path: &Path) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.trim().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("file '{}' is empty", path.display()),
        ));
    }

    Ok(())
}

pub fn ensure_images_are_new(path: &Path, recipes: &[RecipeCompose]) -> io::Result<()> {
    let existing_images = read_existing_images(path)?;
    let mut images_to_add = HashSet::new();

    for recipe in recipes {
        if existing_images.contains(&recipe.image) || !images_to_add.insert(recipe.image.clone()) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Image '{}' already exists in YAML file", recipe.image),
            ));
        }
    }

    Ok(())
}

pub fn ensure_name_is_new(path: &Path, name: &str) -> io::Result<()> {
    if read_existing_names(path)?
        .iter()
        .any(|existing| existing == name)
    {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Config '{}' already exists in env file", name),
        ));
    }

    Ok(())
}

fn read_existing_images(path: &Path) -> io::Result<HashSet<String>> {
    if !path.exists() {
        return Ok(HashSet::new());
    }

    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    if contents.trim().is_empty() {
        return Ok(HashSet::new());
    }

    let recipes: std::collections::HashMap<String, RecipeCompose> = serde_yaml::from_str(&contents)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    Ok(recipes.into_values().map(|recipe| recipe.image).collect())
}

fn read_existing_names(path: &Path) -> io::Result<Vec<String>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    BufReader::new(File::open(path)?)
        .lines()
        .filter_map(|line| match line {
            Ok(value) => value
                .strip_prefix("# devinit config: ")
                .map(|name| Ok(name.trim().to_owned())),
            Err(error) => Some(Err(error)),
        })
        .collect()
}
