#![allow(unused_assignments)]
#![allow(dead_code)]
use libbpf_rs::{Link, MapFlags, ObjectBuilder, Program};
use std::{fs::File, os::fd::IntoRawFd};

use crate::config::config::Config;

pub fn bpf_scaffold(portfd: i32, config: &Config) {
    let mut object = ObjectBuilder::default();
    let open_object = object.open_file("minimal.bpf.o").unwrap();
    let mut obj = open_object.load().unwrap();
    let socket_map = obj.map("socket_map").unwrap();
    let port_map = obj.map("port_map").unwrap();
    let key: &[u8; 4] = &[0, 0, 0, 0];
    let value: &[u8; 8] = &(portfd as u64).to_le_bytes();
    socket_map.update(key, value, MapFlags::ANY).unwrap();
    for val in config.to_owned().servers.unwrap().values_mut() {
        for v in val.ports.as_ref().unwrap() {
            let key: &[u8; 2] = &(*v as u16).to_le_bytes();
            let value: &[u8; 1] = &[0];
            port_map.update(key, value, MapFlags::ANY).unwrap();
        }
    }
    let f = File::open("/proc/self/ns/net").unwrap();
    let rawfd = f.into_raw_fd();
    let program = obj.prog_mut("proxy_dispatch").unwrap();
    let mut link = program.attach_netns(rawfd).unwrap();
    // unpin program if it exist
    unpin_prog(program, "/sys/fs/bpf/prog");
    // unpin link if it exist
    unpin_link(program, "/sys/fs/bpf/hook_point", &mut link);
    program.pin("/sys/fs/bpf/prog").unwrap();
    link.pin("/sys/fs/bpf/hook_point").unwrap();
}

fn unpin_prog(prog: &mut Program, path: &str) {
    match prog.unpin(path) {
        Ok(_) => (),
        Err(e) => {
            if e.to_string().as_str() == "System(2)" {
                prog.pin(path).unwrap();
            } else {
                eprintln!("{}", e);
            }
        }
    }
}
fn unpin_link(prog: &mut Program, path: &str, link: &mut Link) {
    match prog.unpin(path) {
        Ok(_) => (),
        Err(e) => {
            if e.to_string().as_str() == "System(2)" {
                link.pin(path).unwrap();
            } else {
                eprintln!("{}", e);
            }
        }
    }
}
