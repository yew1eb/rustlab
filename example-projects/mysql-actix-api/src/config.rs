pub const CONFIG_FILENAME: &str = "config.toml";

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {
    pub server: ServerConfig,

    pub log: LogConfig,

    pub db: MysqlConfig,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ServerConfig {
    pub port: String,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LogConfig {
    pub actix_web: String,

    pub webapp: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct MysqlConfig {
    pub url: String,
}
