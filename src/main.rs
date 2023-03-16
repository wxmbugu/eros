use eros::config::config::read_config;
fn main() {
    let config = read_config(include_str!("../example/config.toml"));
    println!("{config:#?}");
}
