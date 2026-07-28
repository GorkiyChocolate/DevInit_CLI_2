pub struct ComposeService {
    name: String,
    image: String,
    ports: Vec<Port>,
    environment: Vec<Environment>,
    volumes: Vec<Volume>,
    networks: Vec<String>,
    depends_on: Vec<String>,
    restart: Option<RestartPolicy>,
    healthcheck: Option<Healthcheck>,
    command: Option<Vec<String>>,
}
// struct to appending data in docker-compose file