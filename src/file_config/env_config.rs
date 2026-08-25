use crate::models::structs::RecipeCompose;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn env_file_config(recipe: &RecipeCompose, path: &Path) -> std::io::Result<()> {
    let Some(env_values) = recipe.env.as_ref() else {
        return Ok(());
    };

    if env_values.is_empty() {
        return Ok(());
    }

    let existing_names = read_existing_names(path)?;
    if existing_names.iter().any(|name| name == &recipe.name) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("Config '{}' already exists in env file", recipe.name),
        ));
    }

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    if path.metadata()?.len() > 0 {
        file.write_all(b"\n")?;
    }

    writeln!(file, "# devinit config: {}", recipe.name)?;
    for value in env_values {
        writeln!(file, "{}", value)?;
    }

    Ok(())
}

fn read_existing_names(path: &Path) -> std::io::Result<Vec<String>> {
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
