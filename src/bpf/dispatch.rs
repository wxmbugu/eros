#![allow(unused_assignments)]
#![allow(dead_code)]
use libbpf_rs::{MapFlags, ObjectBuilder};
use std::{fs::File, os::fd::IntoRawFd};

use crate::config::config::Config;

pub fn bpf_scaffold(portfd: i32, config: &Config) {
    let mut object = ObjectBuilder::default();
    let open_object = object.open_file("minimal.bpf.o").unwrap();
    let mut obj = open_object.load().unwrap();
    println!("port fd ={portfd}");
    let socket_map = obj.map("socket_map").unwrap();
    let port_map = obj.map("port_map").unwrap();
    println!("{socket_map:?},{port_map:?}");
    let key: &[u8; 4] = &[0, 0, 0, 0];
    let value: &[u8; 8] = &(portfd as u64).to_le_bytes();
    socket_map.update(key, value, MapFlags::ANY).unwrap();
    for val in config.to_owned().servers.unwrap().values_mut() {
        println!("{:?}", val.ports.clone().unwrap());
        for v in val.ports.as_ref().unwrap() {
            let key: &[u8; 2] = &(*v as u16).to_le_bytes();
            let value: &[u8; 1] = &[0];
            port_map.update(key, value, MapFlags::ANY).unwrap();
        }
    }
    let f = File::open("/proc/self/ns/net").unwrap();
    let rawfd = f.into_raw_fd();
    println!("rawfd  =   {rawfd}");
    let program = obj.prog_mut("proxy_dispatch").unwrap();
    let mut link = program.attach_netns(rawfd).unwrap();
    program.unpin("/sys/fs/bpf/prog").unwrap();
    program.unpin("/sys/fs/bpf/hook_point").unwrap();
    program.pin("/sys/fs/bpf/prog").unwrap();
    link.pin("/sys/fs/bpf/hook_point").unwrap();
    println!("{link:?}");
}
