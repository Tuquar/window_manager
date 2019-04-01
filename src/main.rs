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
            // println!("{}", line);
            if line.starts_with("[") {
                println!("New---");
                    windows.push(create_window(&mut window));
                    // Issue with pointers. You need to learn pass by reference/value more thoroughly
                    window.clear()
                    // window = Vec::new();
            }
            window.push(line);
        }
        println!("Wlength: {}", windows.len());

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
        println!("Lines -> {}", config_lines[0]);
        let window = super::Window {
            label: config_lines.remove(0),
            // name: config_lines.remove(1),
            // monitor_id: config_lines.remove(1),
            // label: config_lines[0].parse().unwrap(),
            name: config_lines[1].parse().unwrap(),
            monitor_id: config_lines[2].parse().unwrap(),
            bits_per_pixel: config_lines[3].parse().unwrap(),
            width: config_lines[4].parse().unwrap(),
            height: config_lines[5].parse().unwrap(),
            display_flags: config_lines[6].parse().unwrap(),
            display_frequency: config_lines[7].parse().unwrap(),
            display_orientation: config_lines[8].parse().unwrap(),
            position_x: config_lines[9].parse().unwrap(),
            position_y: config_lines[10].parse().unwrap(),
        };
        return window;
    }
}