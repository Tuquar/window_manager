extern crate clap;
use clap::{Arg, App};
mod multi_monitor_tool;

#[allow(dead_code)]
pub struct ConfigEntry <V> {
    name: String,
    value: V,
}

#[allow(dead_code)]
pub struct Window {
    label: String,
    name: String,
    monitor_id: String,
    bits_per_pixel: ConfigEntry<u16>,
    width: ConfigEntry<u16>,
    height: ConfigEntry<u16>,
    display_flags: ConfigEntry<u16>,
    display_frequency: ConfigEntry<u16>,
    display_orientation: ConfigEntry<u16>,
    position_x: ConfigEntry<i32>,
    position_y: ConfigEntry<i32>,
}


// cargo run -- -m dev
// cargo run -- -h
// https://rust-lang-nursery.github.io/rust-cookbook/cli/arguments.html#parse-command-line-arguments
fn main() {
    let matches = App::new("Window Manager")
    .version("0.1.0")
    .author("Matthew Lennon")
    .about("Moves and resizes rogue windows to desirable locations")
    .arg(Arg::with_name("mode")
                .short("m")
                .long("mode")
                .takes_value(true)
                .help("dev, games etc"))
    .get_matches();

    let mode = matches.value_of("mode").unwrap();
    
    println!("Selected mode: {}", mode);

    multi_monitor_tool::multi_monitor_tool::get_config();
    let monitors: Vec<Window> = multi_monitor_tool::multi_monitor_tool::parse_config();
    
}

