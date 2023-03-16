#![allow(dead_code)]
use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    app: String,
    servers: Option<HashMap<String, Server>>,
}

#[derive(Debug, Deserialize)]
struct Server {
    targets: Option<Vec<String>>,
    ports: Option<Vec<i32>>,
}

impl Config {
    fn watcher() {
        unimplemented!()
    }
}

pub fn read_config(config: &str) -> Config {
    let decoded: Config = toml::from_str(config).unwrap();
    decoded
}
