extern crate clap;

use clap::{Arg, App};

#[allow(dead_code)]
struct Window {
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
struct ConfigEntry <V> {
    name: String,
    value: V, // TODO make this generic
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

    multi_monitor_tool::get_config();
    multi_monitor_tool::parse_config();
    
}

mod multi_monitor_tool {
    use std::process::Command;
    use std::fs::File;
    use std::io::{BufReader, Result, prelude::*};

    //Docs http://www.nirsoft.net/utils/multi_monitor_tool.html
    const MULTI_MONITOR_EXE: &str = "MultiMonitorTool";
    const CONFIG_LOCATION: &str = ".\\config\\monitorconfig.ini";

    pub fn get_config() {
        Command::new(MULTI_MONITOR_EXE).arg("/Saveconfig").arg(CONFIG_LOCATION).output().expect("");
    }

    pub fn parse_config() {
        let mut windows: Vec<super::Window> = Vec::new();
        let mut window: Vec<String> = Vec::new();

        let config_lines = read_config_file().expect("could not find config file");
        for line in config_lines {
            // println!("L-{}", line);
            if line.starts_with("PositionY") {
                windows.push(create_window(&mut window));
                window.clear();
                // println!("--New Monitor--");
            }
            window.push(line);
        }
        println!("Connected Monitors: {}", windows.len());

        fn read_config_file() ->  Result<Vec<String>>{
            return BufReader::new(File::open(CONFIG_LOCATION)?).lines().collect();
            // let config_file = File::open(CONFIG_LOCATION)?;
            // let mut buf_reader = BufReader::new(config_file);
            // let mut contents = String::new();
            // buf_reader.read_to_string(&mut contents)?;
            // Ok(())
        }
        
            // match config_file {
            //     Ok (x) => print!("ace"),
            //     Err (e) => print!("Poo"),
            // };
    }

    fn create_window(config_lines: &mut Vec<String>) -> super::Window {
        let window = super::Window {
            label: config_lines.remove(0),
            name: config_lines.remove(1),
            monitor_id: config_lines.remove(2),
            bits_per_pixel: parse_entry_u16(config_lines.remove(3)),
            width: parse_entry_u16(config_lines.remove(4)),
            height: parse_entry_u16(config_lines.remove(5)),
            display_flags: parse_entry_u16(config_lines.remove(6)),
            display_frequency: parse_entry_u16(config_lines.remove(7)),
            display_orientation: parse_entry_u16(config_lines.remove(8)),
            position_x: parse_entry_i32(config_lines.remove(9)),
            position_y: parse_entry_i32(config_lines.remove(10)),
        };
        return window;
    }

    fn parse_entry_u16(line: String) -> super::ConfigEntry<u16> {
        let mut sp: Vec<&str> = line.split("=").collect();
        // sp.len() is 1, not parsing the 0 for some reason. "DisplayFlags=0"
        let name: String = sp.remove(0).to_string();
        print!("Line: {}", line);
        let value: u16 = match sp.remove(1).parse::<u16>() {
            Ok(value) => value,
            Err(e) => {
                panic!("Issue parsing config entry! {:?}", e);
            }
        };
        print!("Name: {}, value {}", name, value);
        let entry = super::ConfigEntry {
            name,
            value
        };

        return entry;
    }

    fn parse_entry_i32(line: String) -> super::ConfigEntry<i32> {
        let mut sp: Vec<&str> = line.split("=").collect();
        let name: String = sp.remove(0).to_string();

        let value: i32 = match sp.remove(1).parse::<i32>() {
            Ok(value) => value,
            Err(e) => {
                panic!("Issue parsing config entry! {:?}", e);
            }
        };
        print!("Name: {}, value {}", name, value);
        let entry = super::ConfigEntry {
            name,
            value
        };

        return entry;
    }
}