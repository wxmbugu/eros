#![allow(dead_code)]

use std::{
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

#[derive(Debug)]
pub struct Loadbalancer {
    currentserver: i32,
    pub servers: Vec<String>,
}

impl Loadbalancer {
    pub fn new(servers: Vec<String>) -> Self {
        let servers = servers
            .iter()
            .filter(|x| healthcheck(x))
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();
        Self {
            servers,
            currentserver: 0,
        }
    }

    fn selectserver(&mut self) -> String {
        let mut backend = String::new();
        if !self.servers.is_empty() {
            backend = self
                .servers
                .get(self.currentserver as usize / self.servers.len())
                .unwrap()
                .to_owned();
            self.currentserver += 1
        } else {
            println!("DEBUG:No available servers");
        }
        backend
    }
}

fn healthcheck(uri: &String) -> bool {
    let rr = uri.to_socket_addrs().unwrap().next();
    let result = TcpStream::connect_timeout(&rr.unwrap(), Duration::new(20, 0));
    match result {
        Ok(_) => true,
        Err(_) => {
            println!("DEBUG:unhealthy server");
            false
        }
    }
}
