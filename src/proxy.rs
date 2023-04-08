#![allow(dead_code)]
use std::{collections::HashMap, /* net::TcpListener */ ops::RangeInclusive};

// use tokio::net::TcpStream;

use crate::{
    config::config::{Config, Server},
    lbalancer::Loadbalancer,
};

#[derive(Debug)]
struct Proxy {
    con: String,
    lb: Loadbalancer,
    config: Config,
}

impl Proxy {
    fn new(config: Config) -> Self {
        let con = String::new();
        let lb = Loadbalancer::new(Vec::new());
        Proxy { con, lb, config }
    }
    async fn proxy(&mut self) {}
}

fn server_portrange_map(config: &HashMap<String, Server>) -> HashMap<String, RangeInclusive<i32>> {
    let mut server_port_map = HashMap::<String, RangeInclusive<i32>>::new();
    for (servername, server) in config {
        let start = *server.ports.as_ref().unwrap().first().unwrap();
        let end = *server.ports.as_ref().unwrap().last().unwrap();
        server_port_map.insert(servername.to_owned(), start..=end);
    }
    server_port_map
}
pub fn match_ports_server(dest: &str, config: &HashMap<String, Server>) -> Option<String> {
    let port = dest.split(':').filter_map(|x| x.parse::<i32>().ok()).last();
    let server_portrange_map = server_portrange_map(config);
    for (server, range) in server_portrange_map.iter() {
        match port {
            _ if range.contains(&port.unwrap()) => return Some(server.to_owned()),
            _ => {
                continue;
            }
        };
    }
    None
}
