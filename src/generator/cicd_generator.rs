use std::{fs, path::PathBuf};


pub trait CiCdGenerator{
    fn files_existing(list_of_files: &str) -> bool;
    fn file_text_existence(list_of_files: &str) -> bool;
    fn file_generating() -> Result<(), std::io::Error>;
}

pub struct CiCd_Files{
    pub ci_yaml: String,
    pub build_push: String,
    pub deploy: String,
    pub dir_path: PathBuf,
}

impl CiCdGenerator for CiCd_Files {
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

pub fn cicd_generator() -> Result<(), std::io::Error>{
    let mut dir_path = PathBuf::from(".github/workflow");

    fs::create_dir_all(&dir_path)?;
    let ci_yaml = "ci.yaml";
    let build_push = "build_push.yaml";
    let deploy = "deploy.yaml";

    let files_names = [ci_yaml, build_push, deploy];

    for name in files_names {
        dir_path.push(name);
        fs::File::create(&dir_path)?;
        dir_path.pop();
    }
    Ok(())

}