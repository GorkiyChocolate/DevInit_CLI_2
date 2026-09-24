use std::{fs::{self}, path::{PathBuf}};


pub fn k8s_generator_files() -> Result<(), std::io::Error> {
    let mut dir_path = PathBuf::from("k8s");
    fs::create_dir_all(&dir_path)?;

    let configmap = "config map.yaml";
    let secret = "secret.yaml";
    let deployment = "deployment.yaml";
    let service = "service.yaml";
    let ingress = "ingess.yaml";

    let files_names= [configmap, secret, deployment, service, ingress]; 

    for name in files_names {
        dir_path.push(name);
        fs::File::create(&dir_path)?;

        dir_path.pop();
    }

    Ok(())
}