#![allow(unused_assignments)]
use eros::{config::config::read_config, lb, proxy::match_ports_server};
use libbpf_rs::{MapFlags, ObjectBuilder};
use std::fs::File;
use std::net::TcpListener;
use std::os::fd::IntoRawFd;
use syscalls::{syscall2, syscall3, Sysno};
fn main() {
    let config = read_config(include_str!("../example/config.toml"));
    let server = config.servers.as_ref().unwrap();
    if let Some(server) = server.clone().get("alpha") {
        let lb = lb::lbalancer::Loadbalancer::new(server.targets.to_owned().unwrap());
        println!("{:?}", lb.servers);
    }
    let server_name = match_ports_server("tcp-echo.fly.dev:7001", server);
    println!("{:#?}", server_name);
    let mut object = ObjectBuilder::default();
    let open_object = object.open_file("minimal.bpf.o").unwrap();
    let mut obj = open_object.load().unwrap();
    let program = obj.prog_mut("proxy_dispatch").unwrap();
    let f = File::open("/proc/self/ns/net").unwrap();
    let rawfd = f.into_raw_fd();
    println!("rawfd  =   {rawfd}");
    let link = program.attach_netns(rawfd).unwrap();
    println!("{link:?}");
    let targetpid = 7276;
    let mut targetpidfd: usize = 0;
    unsafe {
        targetpidfd = syscall2(Sysno::pidfd_open, targetpid, 0).unwrap();
        println!("spew bs = {targetpidfd}");
    }
    let targetfd = 3;
    let mut socketfd = 0;
    unsafe {
        socketfd = syscall3(Sysno::pidfd_getfd, targetpidfd, targetfd, 0).unwrap();
        println!("spew bs2 = {socketfd}");
    }
    let map = obj.map("socket_map").unwrap();
    println!("{map:?}");
    let key: &[u8; 4] = &[0, 0, 0, 0];
    let value: &[u8; 8] = &socketfd.to_le_bytes();
    map.update(key, value, MapFlags::ANY).unwrap();
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
