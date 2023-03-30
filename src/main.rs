use eros::{config::config::read_config, lb};
fn main() {
    let config = read_config(include_str!("../example/config.toml"));
    let server = config.servers.unwrap();
    if let Some(server) = server.get("alpha") {
        let lb = lb::lbalancer::Loadbalancer::new(server.targets.to_owned().unwrap());
        println!("{:?}", lb.servers);
    }
}
