use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub fn append_data(text: &str, path:&PathBuf) -> std::io::Result<()> {
    let mut docker_compose = OpenOptions::new()
        .create(true) //create if doesnt exist
        .append(true) //not overwrite file
        .open(&path)?;

    let dependency  = structs; //appending dependency into compose file
    writeln!(file, user)?; 
    Ok(())
} 