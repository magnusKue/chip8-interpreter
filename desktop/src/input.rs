use chip8_core::Emu;

use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

use std::collections::HashMap;
use std::process;

use crate::config::Config;

pub struct InputManager {
    // used to map the array index of the toml var
    // to the hex index which is send to the emulator
    index_to_hex_map: HashMap<u32, u8>,

    // used to map the string keyname of the toml
    // to raylib keycodes 
    name_to_key_map: HashMap<String, KeyboardKey>,

    game_keymap: HashMap<KeyboardKey, u8>,
    emu_keymap: HashMap<String, KeyboardKey>,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            // array indx | chip8 keys
            //    0123        123C
            //    4567   ->   456D
            //    89..        789E
            //    ....        A0BF
            index_to_hex_map: HashMap::from([
                (0,  0x1),
                (1,  0x2),
                (2,  0x3),
                (3,  0xC),

                (4,  0x4),
                (5,  0x5),
                (6,  0x6),
                (7,  0xD),

                (8,  0x7),
                (9,  0x8),
                (10, 0x9),
                (11, 0xE),

                (12, 0xA),
                (13, 0x0),
                (14, 0xB),
                (15, 0xF),
            ]),
            
            // Im sorry for this
            name_to_key_map: HashMap::from([
                ("A".to_string(), KEY_A),
                ("B".to_string(), KEY_B),
                ("C".to_string(), KEY_C),
                ("D".to_string(), KEY_D),
                ("E".to_string(), KEY_E),
                ("F".to_string(), KEY_F),
                ("G".to_string(), KEY_G),
                ("H".to_string(), KEY_H),
                ("I".to_string(), KEY_I),
                ("J".to_string(), KEY_J),
                ("K".to_string(), KEY_K),
                ("L".to_string(), KEY_L),
                ("M".to_string(), KEY_M),
                ("N".to_string(), KEY_N),
                ("O".to_string(), KEY_O),
                ("P".to_string(), KEY_P),
                ("Q".to_string(), KEY_Q),
                ("R".to_string(), KEY_R),
                ("S".to_string(), KEY_S),
                ("T".to_string(), KEY_T),
                ("U".to_string(), KEY_U),
                ("V".to_string(), KEY_V),
                ("W".to_string(), KEY_W),
                ("X".to_string(), KEY_X),
                ("Y".to_string(), KEY_Y),
                ("Z".to_string(), KEY_Z),

                ("0".to_string(), KEY_ZERO),
                ("1".to_string(), KEY_ONE),
                ("2".to_string(), KEY_TWO),
                ("3".to_string(), KEY_THREE),
                ("4".to_string(), KEY_FOUR),
                ("5".to_string(), KEY_FIVE),
                ("6".to_string(), KEY_SIX),
                ("7".to_string(), KEY_SEVEN),
                ("8".to_string(), KEY_EIGHT),
                ("9".to_string(), KEY_NINE),

                ("TAB".to_string(), KEY_TAB),
                ("ESCAPE".to_string(), KEY_ESCAPE),
                ("SPACE".to_string(), KEY_SPACE),
                ("UP".to_string(), KEY_UP),
                ("DOWN".to_string(), KEY_DOWN),
                ("LEFT".to_string(), KEY_LEFT),
                ("RIGHT".to_string(), KEY_RIGHT),
                ("ENTER".to_string(), KEY_ENTER)
            ]),

            // placeholder
            game_keymap: HashMap::from([(KEY_A, 0x0)]),
            emu_keymap: HashMap::from([("".to_string(), KEY_A)])
        }
    }

    pub fn generate_keymaps_from_config(&mut self, config: &Config) {
        let used_keys = self.generate_emu_keymap_from_config(&config);
        self.generate_game_keymap_from_config(&config, used_keys);

        println!("INFO: Generated keymaps from config")
    }

    fn generate_emu_keymap_from_config(&mut self, config: &Config) -> Vec<String>{
        let raw: Vec<Vec<String>> = config.emulator_input.clone();
        let mut emu_map: HashMap<String, KeyboardKey> = HashMap::new();

        // track used keys to check for double assignments
        let mut used = Vec::new();

        for pair in raw {
            println!("{:?}", pair);
            // Bsp: pair = { "RESET", "Z" }

            used.push(pair[1].clone());
            
            emu_map.insert(
                pair[0].clone(), 
                *self.name_to_key_map.get(&pair[1]).expect("df")
            );
        }
        self.emu_keymap = emu_map;
        
        used
    }
    
    fn generate_game_keymap_from_config(&mut self, config: &Config, taken_keys: Vec<String>) {
        let raw: Vec<String> = config.game_input.clone();
        let mut game_map: HashMap<KeyboardKey, u8> = HashMap::new();

        for (index, value) in raw.into_iter().enumerate() {
            if taken_keys.contains(&value) {
                println!("ERROR: Double assigned Key: [{}]!", value);
                process::exit(0);
            }
            game_map.insert(
                *self.name_to_key_map.get(&value).expect("df"), 
                *self.index_to_hex_map.get(&(index as u32)).expect("df")
            );
        }
        self.game_keymap = game_map;
    }

    pub fn handle_game_input(&mut self, emulator: &mut Emu, rl: &RaylibHandle) {

        for (ray_key, key_id) in &self.game_keymap {
            if rl.is_key_down(ray_key.clone()) {
                emulator.keypress(key_id.clone() as usize, true);
            }
            else {
                emulator.keypress(key_id.clone() as usize, false);
            }
        }
    }

    pub fn handle_emu_input(&mut self, rl: &RaylibHandle) -> Vec<String> {
        //fuck the borrow checker!
        let emu_keymap = self.emu_keymap.clone();

        let mut actions_buffer = Vec::new();

        for (action, ray_key) in emu_keymap {
            if rl.is_key_pressed(ray_key.clone()) {
                actions_buffer.push(action);
            }
        };

        actions_buffer
    }

}