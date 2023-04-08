use std::net::TcpListener;

use eros::{config::config::read_config, lb, proxy::match_ports_server};
fn main() {
    let config = read_config(include_str!("../example/config.toml"));
    let server = config.servers.as_ref().unwrap();
    if let Some(server) = server.clone().get("alpha") {
        let lb = lb::lbalancer::Loadbalancer::new(server.targets.to_owned().unwrap());
        println!("{:?}", lb.servers);
    }
    let server_name = match_ports_server("tcp-echo.fly.dev:7001", server);
    println!("{:#?}", server_name);
    let listener = TcpListener::bind("localhost:9001").unwrap();
    println!("Listening on port :9001");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                stream.shutdown(std::net::Shutdown::Both).unwrap();
            }
            Err(e) => println!("{e}"),
        };
    }
}
