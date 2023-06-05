use std::{
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

use crate::{
    config::config::{Config, Server},
    proxy::match_ports_server,
};

#[derive(Debug, Clone)]
pub struct Loadbalancer {
    nextserver: u16,
    pub servers: Vec<String>,
    pub config: Config,
}

impl Loadbalancer {
    pub fn new(config: Config) -> Self {
        Self {
            servers: Vec::new(),
            nextserver: 0,
            config,
        }
    }
    pub fn selectserver(&mut self, port: u16) -> Option<String> {
        let server_name = match_ports_server(port, &self.config).unwrap();
        let targets = self.getserver(&server_name).targets.unwrap();
        self.servers = filter_healthy_server(targets);
        if self.servers.is_empty() {
            return None;
        }
        let backend = self
            .servers
            .get(self.nextserver as usize)
            .unwrap()
            .to_owned();
        self.nextserver = (self.nextserver + 1) % self.servers.len() as u16;
        Some(backend)
    }
    fn getserver(&self, server_name: &String) -> Server {
        self.config
            .servers
            .as_ref()
            .unwrap()
            .get(server_name)
            .unwrap()
            .clone()
    }
}
fn filter_healthy_server(targets: Vec<String>) -> Vec<String> {
    targets
        .iter()
        .filter(|x| healthcheck(x))
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
}
fn healthcheck(uri: &String) -> bool {
    let rr = uri.to_socket_addrs().unwrap().next();
    let result = TcpStream::connect_timeout(&rr.unwrap(), Duration::new(20, 0));
    result.is_ok()
}
