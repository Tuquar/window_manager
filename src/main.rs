extern crate clap;

use clap::{Arg, App};

#[allow(dead_code)]
struct Window {
    label: String,
    name: String,
    monitor_id: String,
    bits_per_pixel: u8,
    width: u8,
    height: u8,
    display_flags: u8,
    display_frequency: u8,
    display_orientation: u8,
    position_x: u8,
    position_y: u8,
}

#[allow(dead_code)]
struct ConfigEntry {
    name: String,
    value: u8, // TODO make this generic
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
            // name: config_lines.remove(1),
            // monitor_id: config_lines.remove(2),
            // bits_per_pixel: config_lines.remove(3).parse().unwrap(),
            // width: config_lines.remove(4).parse().unwrap(),
            // height: config_lines.remove(5).parse().unwrap(),
            // display_flags: config_lines.remove(6).parse().unwrap(),
            // display_frequency: config_lines.remove(7).parse().unwrap(),
            // display_orientation: config_lines.remove(8).parse().unwrap(),
            // position_x: config_lines.remove(9).parse().unwrap(),
            // position_y: config_lines.remove(10).parse().unwrap(),

            name: "".to_string(),
            monitor_id: "".to_string(),
            bits_per_pixel: 1,
            width: 1,
            height: 1,
            display_flags: 1,
            display_frequency: 1,
            display_orientation: 1,
            position_x: 1,
            position_y: 1,
            
        };
        return window;
    }
}