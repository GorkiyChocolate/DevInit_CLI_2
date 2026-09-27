use crate::models::cicd_struct::Pipeline;

pub fn cicd_validator(cicd: &Pipeline) -> Result<(), std::io::Error>{
    println!("pipeline: {:?}", cicd);
    Ok(())
}