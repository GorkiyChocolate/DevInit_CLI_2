use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::structs::RecipeCompose;

pub fn append_data(config_struct: &RecipeCompose, path: &PathBuf) -> std::io::Result<()> {
    let file_path = path;

    let mut recipes_map: HashMap<String, RecipeCompose> = HashMap::new();

    if file_path.exists() {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        if !contents.trim().is_empty() {
            recipes_map = serde_yaml::from_str(&contents)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        }
    }

    println!("Было сервисов в файле: {:?}", recipes_map.keys().collect::<Vec<_>>());

    recipes_map.insert(config_struct.name.clone(), config_struct.clone());

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)?;

    let updated_yaml = serde_yaml::to_string(&recipes_map)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        
    file.write_all(updated_yaml.as_bytes())?;

    println!("Сервис '{}' успешно добавлен/обновлен в конфигурации.", config_struct.name);
    Ok(())
}
