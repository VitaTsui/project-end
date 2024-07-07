use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref CONFIG: Settings = Settings::default();
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub mysql: Database,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Default for Settings {
    fn default() -> Self {
        let yaml_data = std::fs::read_to_string(".yaml").unwrap();
        serde_yaml::from_str::<Settings>(&yaml_data).unwrap()
    }
}
