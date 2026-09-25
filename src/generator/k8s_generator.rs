use std::{fs::{self}, path::{PathBuf}};

pub trait K8sGenerator{
    fn files_existing(list_of_files: &str) -> bool;
    fn file_text_existence(list_of_files: &str) -> bool;
    fn file_generating() -> Result<(), std::io::Error>;
}

pub struct K8s_Files{
    pub configmap: String,
    pub secret: String,
    pub deployment: String,
    pub service: String,
    pub ingress: String,
    pub dir_path: PathBuf,
}

impl K8sGenerator for K8s_Files {
    fn file_generating() -> Result<(), std::io::Error> {
        todo!()
    }

    fn file_text_existence(list_of_files: &str) -> bool {
        todo!()
    }

    fn files_existing(list_of_files: &str) -> bool {
        todo!()
    }
}

pub fn k8s_generator_files() -> Result<(), std::io::Error> {
    let mut dir_path = PathBuf::from("k8s");
    fs::create_dir_all(&dir_path)?;

    let configmap = "configmap.yaml";
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