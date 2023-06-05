#![allow(unused_assignments)]
use std::net::TcpListener;
use std::os::fd::AsRawFd;

use eros::config::config;
use eros::dispatch::bpf_scaffold;
use eros::proxy::handlestream;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:9001").unwrap();
    println!("Listening on port :9001");
    let td = listener.as_raw_fd();
    println!("portfd = {td}");
    let config = config::read_config(include_str!("../example/config.toml"));
    bpf_scaffold(td, &config);
    for stream in listener.incoming() {
        let tokiostream = TcpStream::from_std(stream.unwrap()).unwrap();
        handlestream(tokiostream, &config).await;
    }
}
