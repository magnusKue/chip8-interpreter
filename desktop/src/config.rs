use serde::Deserialize;
use std::fs;
use std::process;


pub const VALID_ACTIONS: [&str; 7] = [
    "PAUSE",
    "RESET",
    "NEXT_THEME",
    "EXIT",
    "LOAD",
    "SAVE",
    "HONK",
];

#[derive(Deserialize, Debug)]
pub struct Config {
    pub themes: Vec<Vec<String>>,  
    
    pub show_path: bool,
    pub show_fps: bool,
    
    pub max_fps: u32,
    pub tps: u32,

    pub frequency: f32,
    pub duration: f32,
    pub volume: f32,

    pub game_input: Vec<String>,
    pub emulator_input: Vec<Vec<String>>,
}

pub fn read_config() -> Config {
    // Read the TOML file
    let config_content = fs::read_to_string("config.toml").expect("ERROR: Failed to read config file");
    // Parse the TOML content
    let conf: Config = toml::from_str(&config_content).expect("ERROR: Failed to parse config file");
    println!("INFO: config.toml read successfully!");

    for vec in conf.emulator_input.iter() {
        if !VALID_ACTIONS.contains(&vec[0].as_str()) {
            println!("ERROR: Invalid action defined");
            process::exit(0);
        }
    }
    
    conf
}

#[allow(unused)]
pub fn get_readable_action_name(action: &str) -> &str {
    // radable versions of the actions for
    // the pause menu.
    match action {
        "EXIT" => { "exit game" },
        "LOAD" => { "load savestate" },
        "SAVE" => { "create savestate"},
        "NEXT_THEME" => { "change theme" },
        "HONK" => { "honk" },
        "RESET" => { "reset the game" },
        "PAUSE" => { "continue" }
        _ => {
            println!("ERROR: Invalid action");
            "INVALID ACTION"
        }
    }
}

pub fn _get_action_key_name(conf: &Config, action: String) -> String {
    let mut key: String = "None".to_string();

    for pair in conf.emulator_input.clone() {
        if pair[0] == action {
            key = pair[1].clone();
        }
    };

    key
}
