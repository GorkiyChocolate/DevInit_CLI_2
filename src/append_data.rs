use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub fn append_data(text: &str, path:&PathBuf, target_name: &str) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            // Check if it matches the target file name
            if path.is_file() && path.file_name().map_or(false, |name| name == target_name) {
                return Ok(true);
            }
        }
    }

    let mut docker_compose = OpenOptions::new()
        .create(true) //create if doesnt exist
        .append(true) //not overwrite file
        .open(&path)?;

    let dependency  = structs; //appending service into compose file
    writeln!(file, user)?; 
    Ok(())
} 