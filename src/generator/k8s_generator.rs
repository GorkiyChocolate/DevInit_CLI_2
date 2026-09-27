use std::{fs::{self}, path::{PathBuf}};

pub fn k8s_generator_files() -> Result<(), std::io::Error> {
    let mut dir_path = PathBuf::from("k8s");

    if !dir_path.exists() {
        fs::create_dir_all(&dir_path)?;
    }

    let files_names= ["configmap.yaml", "secret.yaml", "deployment.yaml", "service.yaml", "ingess.yaml"]; 

    for name in files_names {
        dir_path.push(name);
        fs::File::create(&dir_path)?;

        dir_path.pop();
    }

    Ok(())
}