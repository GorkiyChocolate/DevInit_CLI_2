use crate::validator::services_validator;
pub enum Services{
    PostgreSQL,
    MySQL,
    MongoDB,
    MariaDB,
    Elasticsearch,
    Prometheus,
    Grafana,
    Redis,
    RabbitMQ,
    Kafka,
    S3,
    EC2,
}
pub fn services_selector(services: Services) -> Result<(), std::io::Error>{
    match services {
        Services::Elasticsearch => {
            services_validator::validate_elasticsearch_service();
            println!("Elasticsearch service selected");
        },
        Services::Grafana => {
            services_validator::validate_grafana_service();
            println!("Grafana service selected");
        },
        Services::Redis => {
            services_validator::validate_redis_service();
            println!("Redis service selected");
        },
        Services::RabbitMQ => {
            services_validator::validate_rabbitmq_service();
            println!("RabbitMQ service selected");
        },
        Services::Kafka => {
            services_validator::validate_kafka_service();
            println!("Kafka service selected");
        },
        Services::S3 => {
            services_validator::validate_s3_service();
            println!("S3 service selected");
        },
        Services::EC2 => {
            services_validator::validate_ec2_service();
            println!("EC2 service selected");
        },
        _ => {
            println!("Service not supported for validation");
        }
    }
    Ok(())
}