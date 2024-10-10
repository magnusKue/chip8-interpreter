use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub themes: Vec<Vec<String>>,  
    pub show_path: bool,
    pub show_fps: bool,
    pub max_fps: u32,
    pub tps: u32,
    pub game_input: Vec<String>,
    pub emulator_input: Vec<Vec<String>>,
}

pub fn read_config() -> Config {
    // Read the TOML file
    let config_content = fs::read_to_string("config.toml").expect("ERROR: Failed to read config file");
    // Parse the TOML content
    let conf = toml::from_str(&config_content).expect("ERROR: Failed to parse config file");
    println!("INFO: config.toml read successfully!");
    
    conf
}
