use crate::errors::ValidationError;
use crate::models::cicd_struct::Pipeline;
use std::collections::HashSet;

pub fn validate_pipeline(cicd: &Pipeline, errors: &mut Vec<ValidationError>) {
    let mut jobs = HashSet::new();
    for (job_index, job) in cicd.jobs.iter().enumerate() {
        let path = format!("cicd.jobs[{job_index}]");
        if job.name.trim().is_empty() {
            errors.push(ValidationError {
                path: format!("{path}.name"),
                message: "cannot be empty".to_string(),
            });
        }
        if !jobs.insert(job.name.as_str()) {
            errors.push(ValidationError {
                path: format!("{path}.name"),
                message: "job name must be unique".to_string(),
            });
        }
        for (step_index, step) in job.steps.iter().enumerate() {
            let step_path = format!("{path}.steps[{step_index}]");
            if step.name.trim().is_empty() {
                errors.push(ValidationError {
                    path: format!("{step_path}.name"),
                    message: "cannot be empty".to_string(),
                });
            }
            if let crate::models::cicd_struct::StepAction::Run { command } = &step.action
                && command.trim().is_empty()
            {
                errors.push(ValidationError {
                    path: format!("{step_path}.command"),
                    message: "cannot be empty".to_string(),
                });
            }
        }
    }
    for (job_index, job) in cicd.jobs.iter().enumerate() {
        for (need_index, need) in job.needs.iter().enumerate() {
            if !jobs.contains(need.as_str()) {
                errors.push(ValidationError {
                    path: format!("cicd.jobs[{job_index}].needs[{need_index}]"),
                    message: format!("job '{need}' does not exist"),
                });
            }
        }
    }
}

pub fn cicd_validator(cicd: &Pipeline) -> Result<(), std::io::Error> {
    let mut errors = Vec::new();
    validate_pipeline(cicd, &mut errors);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("{} validation error(s)", errors.len()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::cicd_struct::{Job, Runner, Step, StepAction};
    use std::collections::BTreeMap;

    #[test]
    fn reports_missing_job_dependency() {
        let pipeline = Pipeline {
            provider: crate::models::cicd_struct::CiProvider::Github,
            triggers: Vec::new(),
            jobs: vec![Job {
                name: "deploy".to_string(),
                runner: Runner::default(),
                needs: vec!["build".to_string()],
                steps: vec![Step {
                    name: "deploy".to_string(),
                    action: StepAction::Run {
                        command: "deploy".to_string(),
                    },
                    env: BTreeMap::new(),
                }],
            }],
        };
        let mut errors = Vec::new();
        validate_pipeline(&pipeline, &mut errors);
        assert!(
            errors
                .iter()
                .any(|error| error.message.contains("does not exist"))
        );
    }
}
