use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub hosts: Vec<String>,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub connectionPool: ConnectionPool,
    pub serializer: Serializer,
}

#[derive(Debug, Deserialize)]
pub struct ConnectionPool {
    pub enableSsl: bool,
    pub sslEnabledProtocols: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Serializer {
    pub className: String,
    pub config: SerializerConfig,
}

#[derive(Debug, Deserialize)]
pub struct SerializerConfig {
    pub serializeResultToString: bool,
}
