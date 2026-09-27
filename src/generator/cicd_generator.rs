use std::{fs, path::PathBuf};


pub fn cicd_generator() -> Result<(), std::io::Error>{
    let mut dir_path = PathBuf::from(".github/workflow");

    if !dir_path.exists() {
        fs::create_dir_all(&dir_path)?;
    }

    let files_names = ["ci.yaml", "build_push.yaml", "deploy.yaml"];

    for name in files_names {
        dir_path.push(name);
        fs::File::create(&dir_path)?;
        dir_path.pop();
    }
    Ok(())

}