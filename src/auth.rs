use std::env;

pub struct Auth {
    pub client_id: String,
    pub secret: String,
}
impl Auth {
    pub fn new(service_name: &str) -> Auth {
        let get = |key: &str| {
            env::var(format!("{}_{}", service_name, key))
                .expect("Expected key '{}', for service '{}',\
                 but it was not found within the current environment variables!")
        };
        Self {
            client_id: get("ID"),
            secret: get("SECRET"),
        }
    }
}
