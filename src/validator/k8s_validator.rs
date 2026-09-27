use crate::models::k8s_struct::Deployment;

pub fn validate_deployment(deployment: &Deployment) -> Result<(), std::io::Error> {
    // Implementation for validating the deployment
    println!("deployment: {:?}", deployment);
    Ok(())
}