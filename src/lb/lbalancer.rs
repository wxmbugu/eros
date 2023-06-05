use std::{
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

#[derive(Debug)]
pub struct Loadbalancer {
    currentserver: usize,
    pub servers: Vec<String>,
}

impl Loadbalancer {
    pub fn new(servers: &[String]) -> Self {
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
    pub fn selectserver(&mut self) -> Option<String> {
        if self.servers.is_empty() {
            return None;
        }
        let backend = self.servers.get(self.currentserver).unwrap().to_owned();
        self.currentserver += 1;
        Some(backend)
    }
}

fn healthcheck(uri: &String) -> bool {
    let rr = uri.to_socket_addrs().unwrap().next();
    let result = TcpStream::connect_timeout(&rr.unwrap(), Duration::new(20, 0));
    result.is_ok()
}
