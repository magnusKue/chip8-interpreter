use chip8_core::*;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

use crate::config;
use crate::graphics::GraphicsManager;
use crate::input::InputManager;
use crate::audio::AudioManager;

const WELCOME: &str = r#"
        The Rust
        ______ _______ _______ ______ ______ 
       |      |   |   |_     _|   __ \  __  |
       |   ---|       |_|   |_|    __/  __  |
       |______|___|___|_______|___|  |______|
                                   Emulator

"#;

pub struct AppManager {

    emulator: Emu,
    clock_timer: f32,
    
    config: config::Config,
    audio_manager: AudioManager,
    input_manager: InputManager,
    graphics_manager: GraphicsManager,

    args: Vec<String>,
}

impl AppManager {
    pub fn new() -> Self {

        // print welcome message
        println!("\x1b[1;31m {} \x1b[0m", WELCOME);
        
        let arguments: Vec<_> = env::args().collect();
            if arguments.len() != 2 {
                println!("ERROR: invalid args!");
                process::exit(0);
            }
        
        let mut instance = AppManager {
            
            clock_timer: 0.,
            emulator: Emu::new(),
            
            config: config::read_config(),
            input_manager: InputManager::new(),
            audio_manager: AudioManager::new(),
            graphics_manager: GraphicsManager::new(),

            args: arguments,
        };
        instance.audio_manager.load_values_from_config(&instance.config);
        instance.audio_manager.play_async_beep();
        instance.input_manager.generate_keymaps_from_config(&instance.config);
        instance.graphics_manager.theme_manager.parse_themes(&instance.config);

        instance.graphics_manager.rl.set_target_fps(instance.config.max_fps);
        instance.graphics_manager.canvas.clear_background(instance.graphics_manager.get_ui_col("BG".to_string()));
        instance.graphics_manager.canvas.draw_text("LOADING..", 1, 3, 13, instance.graphics_manager.get_ui_col("TEXT".to_string()));
        instance
    }

    pub fn main_loop(&mut self) {
        while !self.graphics_manager.rl.window_should_close() {

            let mut visuals_modified = self.emulator.tick();
            
            if self.update_clocks() {
                // BEEP
                self.audio_manager.play_async_beep();
            }

        
            self.input_manager.handle_game_input(&mut self.emulator, &self.graphics_manager.rl);
            let actions = self.input_manager.handle_emu_input(&self.graphics_manager.rl);
            visuals_modified |= self.handle_action(actions);
    
            self.graphics_manager.render_game(&self.args, &self.config, &self.emulator, visuals_modified);
        }
    }

        
    fn update_clocks(&mut self) -> bool{
        self.clock_timer += self.graphics_manager.rl.get_frame_time();
        
        let mut beep = false;

        if self.clock_timer >= 1. / (self.config.tps as f32) {
            beep = self.emulator.tick_timers();
            self.clock_timer = 0.;
        }

        beep
    }

    pub fn load_rom(&mut self, path: Option<String>) {
        let mut load_path = self.args[1].clone();
        if path.is_some() {
            load_path = path.expect("WTF: You really shouldnt see this");
        }

        let mut rom = File::open(load_path).expect("ERROR: Unable to open file");
        let mut buffer = Vec::new();

        rom.read_to_end(&mut buffer).expect("ERROR: File could not be read");
        // println!("buffer: {:?}", buffer);

        self.emulator.load(&buffer);
        println!("INFO: Loaded ROM successfully");
    }

    fn handle_action(&mut self, actions: Vec<String>) ->bool {
        let mut visuals_modified = false;
        for action in actions {
            match action.as_str() {
                "RESET" => {
                    println!("ACTION: RESET EMULATOR");
                    self.emulator.reset();
                    self.load_rom(None);
                },
                "NEXT_THEME" => {
                    println!("ACTION: Switched theme");
                    self.graphics_manager.next_theme();
                    visuals_modified = true;
                },
                "HONK" => {
                    println!("HONK: HONK HONK!");
                    self.audio_manager.play_async_beep();
                }
                
                "EXIT" => {
                    println!("ACTION: Exiting game");
                    process::exit(0);
                }
                "PAUSE" => {
                    self.emulator.is_paused ^= true;
                }
                _ => {
                    print!("ERROR: Unimplemented ACTION");
                }
            }
        };
        visuals_modified
    }
}

