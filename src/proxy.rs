#![allow(dead_code)]
use std::{collections::HashMap, io::Result, net::ToSocketAddrs, ops::RangeInclusive};

use crate::{config::config::Config, lbalancer::Loadbalancer};
use tokio::{
    io::{self, AsyncWriteExt},
    net::TcpStream as tokiostream,
};
#[derive(Debug)]
struct Proxy {
    con: String,
    lb: Loadbalancer,
    config: Config,
}

impl Proxy {
    fn new(config: Config, servers: &[String]) -> Self {
        let con = String::new();
        let lb = Loadbalancer::new(servers);
        Proxy { con, lb, config }
    }
    async fn proxy(mut self, inbound: tokiostream) -> Result<()> {
        let server_ip_addr = self.lb.selectserver().unwrap().to_socket_addrs().unwrap();
        let ipv4_addresses: Vec<_> = server_ip_addr.filter(|addr| addr.is_ipv4()).collect();
        let ipv4 = ipv4_addresses.first().unwrap().to_string();
        if let Ok(outbound) = tokiostream::connect(ipv4).await {
            match proxy_stream(inbound, outbound).await {
                Ok(_) => (),
                Err(e) => {
                    println!("Error proxying stream: {:?}", e);
                }
            }
        };
        Ok(())
    }
}

fn server_portrange_map(config: &Config) -> HashMap<String, RangeInclusive<u16>> {
    let mut server_port_map = HashMap::<String, RangeInclusive<u16>>::new();
    for (servername, server) in config.servers.clone().unwrap().iter() {
        let start = *server.ports.as_ref().unwrap().first().unwrap() as u16;
        let end = *server.ports.as_ref().unwrap().last().unwrap() as u16;
        server_port_map.insert(servername.to_owned(), start..=end);
    }
    server_port_map
}
pub fn match_ports_server(port: u16, config: &Config) -> Option<String> {
    let server_portrange_map = server_portrange_map(config);
    for (server, range) in server_portrange_map.iter() {
        match port {
            _ if range.contains(&port) => return Some(server.to_owned()),
            _ => {
                continue;
            }
        };
    }
    None
}
pub async fn handlestream(mut stream: tokiostream, config: &Config) {
    if let Some(server) = match_ports_server(stream.local_addr().unwrap().port(), config) {
        let servers = config.servers.as_ref().unwrap().get(&server).unwrap();
        let proxy = Proxy::new(config.clone(), servers.targets.as_ref().unwrap());
        proxy.proxy(stream).await.unwrap();
    } else {
        stream
            .write_all(b"no proxying on this port\n")
            .await
            .unwrap();
    }
}

async fn proxy_stream(mut inbound: tokiostream, mut outbound: tokiostream) -> Result<()> {
    println!(
        "proxying from =  {:?}, to ={:?}",
        inbound.peer_addr(),
        outbound.peer_addr()
    );
    let (mut ro, mut wo) = outbound.split();
    let (mut ri, mut wi) = inbound.split();
    let client_to_server = async {
        match io::copy(&mut ri, &mut wo).await {
            Ok(_) => wo.shutdown().await,
            Err(e) => Err(e),
        }
    };

    let server_to_client = async {
        match io::copy(&mut ro, &mut wi).await {
            Ok(_) => wi.shutdown().await,
            Err(e) => Err(e),
        }
    };
    match tokio::try_join!(client_to_server, server_to_client) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
