extern crate clap;
extern crate xml;

use clap::{Arg, App};
mod multi_monitor_tool;

#[allow(dead_code)]
pub struct ConfigEntry <V> {
    name: String,
    value: V,
}

#[allow(dead_code)]
pub struct Monitor {
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

#[allow(dead_code)]
pub struct Window {
    name: String,
    process_name: String,
    width: u16,
    height: u16,
    offset_left: u16,
    offset_top: u16,
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
    let monitors: Vec<Monitor> = multi_monitor_tool::multi_monitor_tool::parse_config();
    
    let windows: Vec<Window> = load_window_configs();
}



fn load_window_configs() -> Vec<Window>  {
    use std::fs::read_dir;
    let window_configs = read_dir("configs\\window_configs").unwrap();
    let mut windows: Vec<Window> = Vec::new();
    for path in window_configs {
        let path_label = path.unwrap();
        println!("Config found: {}", path_label.path().display());
        windows.push(config_to_window(path_label.path()));
    }
    println!("Found {} window config(s)", windows.len());
    return windows;
}

fn config_to_window(file_path: std::path::PathBuf) -> Window {
    use std::fs::File;
    use std::io::BufReader;
    use xml::reader::{EventReader, XmlEvent};

    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let window_tag = "window".to_string();
    // let mut depth = 0;
    let mut read = false;
    let mut data: Vec<String> = Vec::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                // println!("{}+{}", indent(depth), name);
                // depth += 1;
                if name.to_string() == window_tag {
                    read = true;
                }
            }
            Ok(XmlEvent::Characters(c)) => {
                // println!("{}-{}", indent(depth), c);
                if read {
                    data.push(c);
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                // println!("{}-{}", indent(depth), name);
                // depth -= 1;
                if name.to_string() == window_tag {
                    read = false;
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    fn parse_u16(n: String) -> u16 {
        return match n.parse::<u16>() {
            Ok(value) => value,
            Err(e) => {
                panic!("Issue parsing config entry! {:?}", e);
            }
        };
    };

    let win = Window {
        name: data.remove(0),
        process_name: data.remove(0),
        width: parse_u16(data.remove(0)),
        height: parse_u16(data.remove(0)),
        offset_left: parse_u16(data.remove(0)),
        offset_top: parse_u16(data.remove(0)),
    };
    print_window(&win);
    return win;
}

fn print_window(w: &Window) {
    println!("{} name: {}", indent(1), w.name);
    println!("{} process_name: {}", indent(1), w.process_name);
    println!("{} width: {}", indent(1), w.width);
    println!("{} height: {}", indent(1), w.height);
    println!("{} offset_left: {}", indent(1), w.offset_left);
    println!("{} offset_top: {}", indent(1), w.offset_top);
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}