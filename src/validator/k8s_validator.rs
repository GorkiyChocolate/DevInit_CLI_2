use crate::models::k8s_struct::Deployment;

trait K8s_Validator {
    fn validate(&self) -> bool;
}

impl K8s_Validator for Deployment {
    fn validate(&self) -> bool {
        // Implementation for validating the deployment
        true
    }
}
