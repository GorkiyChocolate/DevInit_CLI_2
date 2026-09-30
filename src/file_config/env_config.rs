use crate::file_config::file_validator::ensure_name_is_new;
use crate::models::docker_compose_struct::RecipeCompose;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn env_file_config(recipe: &RecipeCompose, path: &Path) -> std::io::Result<()> {
    let Some(env_values) = recipe.env.as_ref() else {
        return Ok(());
    };

    if env_values.is_empty() {
        return Ok(());
    }

    ensure_name_is_new(path, &recipe.name)?;

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
