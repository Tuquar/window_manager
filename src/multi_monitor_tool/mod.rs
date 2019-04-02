use Window;
use ConfigEntry;

pub mod multi_monitor_tool {
    use std::process::Command;
    use std::fs::File;
    use std::io::{BufReader, Result, prelude::*};

    //Docs http://www.nirsoft.net/utils/multi_monitor_tool.html
    const MULTI_MONITOR_EXE: &str = "MultiMonitorTool";
    const CONFIG_LOCATION: &str = ".\\config\\monitorconfig.ini";

    fn read_config_file() ->  Result<Vec<String>>{
        return BufReader::new(File::open(CONFIG_LOCATION)?).lines().collect();
    }
    pub fn get_config() {
        Command::new(MULTI_MONITOR_EXE).arg("/Saveconfig").arg(CONFIG_LOCATION).output().expect("");
    }
    pub fn parse_config() -> Vec<super::Window> {
        let mut windows: Vec<super::Window> = Vec::new();
        let mut window: Vec<String> = Vec::new();

        let config_lines = read_config_file().expect("could not find config file");
        for line in config_lines {
            // println!("L-{}", line);
            window.push(line);
            let mut new_monitor = false;
            match window.last() {
                Some(l) => {
                    if l.starts_with("PositionY") {
                        new_monitor = true;
                    }
                },
                None => continue
            }

            if new_monitor {
                // println!("--New Monitor--");
                windows.push(create_window(&mut window));
                window.clear();
            }
        }
        println!("Connected Monitors: {}", windows.len());
        return windows;
    }

    fn create_window(config_lines: &mut Vec<String>) -> super::Window {
        let window = super::Window {
            label: config_lines.remove(0),
            name: config_lines.remove(0),
            monitor_id: config_lines.remove(0),
            bits_per_pixel: parse_entry_u16(config_lines.remove(0)),
            width: parse_entry_u16(config_lines.remove(0)),
            height: parse_entry_u16(config_lines.remove(0)),
            display_flags: parse_entry_u16(config_lines.remove(0)),
            display_frequency: parse_entry_u16(config_lines.remove(0)),
            display_orientation: parse_entry_u16(config_lines.remove(0)),
            position_x: parse_entry_i32(config_lines.remove(0)),
            position_y: parse_entry_i32(config_lines.remove(0)),
        };
        return window;
    }

    fn parse_entry_u16(line: String) -> super::ConfigEntry<u16> {
        let mut sp: Vec<&str> = line.split("=").collect();
        let name: String = sp.remove(0).to_string();
        let value: u16 = match sp.remove(0).parse::<u16>() {
            Ok(value) => value,
            Err(e) => {
                panic!("Issue parsing config entry! {:?}", e);
            }
        };
        let entry = super::ConfigEntry {
            name,
            value
        };

        return entry;
    }

    fn parse_entry_i32(line: String) -> super::ConfigEntry<i32> {
        let mut sp: Vec<&str> = line.split("=").collect();
        let name: String = sp.remove(0).to_string();

        let value: i32 = match sp.remove(0).parse::<i32>() {
            Ok(value) => value,
            Err(e) => {
                panic!("Issue parsing config entry! {:?}", e);
            }
        };
        let entry = super::ConfigEntry {
            name,
            value
        };

        return entry;
    }
}