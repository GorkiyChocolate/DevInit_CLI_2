use crate::errors::ValidationError;
use crate::models::docker_compose_struct::RecipeCompose;

pub fn validate_service(name: &str, service: &RecipeCompose, errors: &mut Vec<ValidationError>) {
    let base = format!("services.{name}");
    if name.trim().is_empty() {
        errors.push(ValidationError {
            path: "services".to_string(),
            message: "service name cannot be empty".to_string(),
        });
    }
    if service.image.trim().is_empty() {
        errors.push(ValidationError {
            path: format!("{base}.image"),
            message: "cannot be empty".to_string(),
        });
    }
    if let Some(ports) = &service.ports {
        for (index, port) in ports.iter().enumerate() {
            validate_port(port, &format!("{base}.ports[{index}]"), errors);
        }
    }
    if let Some(values) = &service.environment {
        for (index, value) in values.iter().enumerate() {
            if value.trim().is_empty() || value.split_once('=').is_none() {
                errors.push(ValidationError {
                    path: format!("{base}.environment[{index}]"),
                    message: "must contain a non-empty KEY=VALUE pair".to_string(),
                });
            }
        }
    }
    if let Some(values) = &service.volumes {
        for (index, value) in values.iter().enumerate() {
            if value.trim().is_empty() {
                errors.push(ValidationError {
                    path: format!("{base}.volumes[{index}]"),
                    message: "cannot be empty".to_string(),
                });
            }
        }
    }
    if let Some(values) = &service.networks {
        for (index, value) in values.iter().enumerate() {
            if value.trim().is_empty() {
                errors.push(ValidationError {
                    path: format!("{base}.networks[{index}]"),
                    message: "cannot be empty".to_string(),
                });
            }
        }
    }
    if let Some(values) = &service.depends_on {
        for (index, value) in values.iter().enumerate() {
            if value.trim().is_empty() {
                errors.push(ValidationError {
                    path: format!("{base}.depends_on[{index}]"),
                    message: "service name cannot be empty".to_string(),
                });
            }
        }
    }
    if let Some(file) = &service.files
        && file.path.trim().is_empty()
    {
        errors.push(ValidationError {
            path: format!("{base}.files.path"),
            message: "cannot be empty".to_string(),
        });
    }
}

fn validate_port(value: &str, path: &str, errors: &mut Vec<ValidationError>) {
    let port = value.rsplit(':').next().unwrap_or_default();
    match port.parse::<u16>() {
        Ok(0) | Err(_) => errors.push(ValidationError {
            path: path.to_string(),
            message: "port must be a number greater than 0".to_string(),
        }),
        Ok(_) => {}
    }
}

pub fn validate_postgresql_service() -> bool {
    // Implementation for validating PostgreSQL service
    true
}

pub fn validate_mysql_service() -> bool {
    // Implementation for validating MySQL service
    true
}

pub fn validate_mongodb_service() -> bool {
    // Implementation for validating MongoDB service
    true
}

pub fn validate_mariadb_service() -> bool {
    // Implementation for validating MariaDB service
    true
}

pub fn validate_elasticsearch_service() -> bool {
    // Implementation for validating Elasticsearch service
    true
}

pub fn validate_prometheus_service() -> bool {
    // Implementation for validating Prometheus service
    true
}

pub fn validate_grafana_service() -> bool {
    // Implementation for validating Grafana service
    true
}

pub fn validate_redis_service() -> bool {
    // Implementation for validating Redis service
    true
}

pub fn validate_rabbitmq_service() -> bool {
    // Implementation for validating RabbitMQ service
    true
}

pub fn validate_kafka_service() -> bool {
    // Implementation for validating Kafka service
    true
}

pub fn validate_s3_service() -> bool {
    // Implementation for validating S3 service
    true
}

pub fn validate_ec2_service() -> bool {
    // Implementation for validating EC2 service
    true
}
