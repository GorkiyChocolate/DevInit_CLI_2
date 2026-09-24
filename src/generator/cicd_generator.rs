use std::{fs, path::PathBuf};


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