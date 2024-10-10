use chip8_core::*;
use raylib::prelude::*;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process;
use std::collections::HashMap;

use crate::config;
use crate::input::InputManager;
use crate::themes::*;

const WIN_SCALE_FAC: u32 = 15;
const WIN_WIDTH: u32 = (SCREEN_WIDTH as u32) * WIN_SCALE_FAC;
const WIN_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * WIN_SCALE_FAC;

const WELCOME: &str = r#"
        The Rust
        ______ _______ _______ ______ ______ 
       |      |   |   |_     _|   __ \  __  |
       |   ---|       |_|   |_|    __/  __  |
       |______|___|___|_______|___|  |______|
                                   Emulator

"#;

pub struct AppManager {

    clock_timer: f32,
    
    rl: RaylibHandle,
    thread: RaylibThread,
    
    config: config::Config,
    theme_manager: ThemeManager,

    canvas: Image,
    
    emulator: Emu,
    input_manager: InputManager,
    args: Vec<String>,
}

impl AppManager {
    pub fn new() -> Self {

        // print welcome message
        println!("\x1b[1;31m {} \x1b[0m", WELCOME);
        
        // init raylib window
        let (raylib_handle, raylib_thread) = raylib::init()
            .resizable()
            .size(WIN_WIDTH as i32, WIN_HEIGHT as i32)
            .title("Chip8 emulator")
            .build();

        raylib_handle.set_trace_log(TraceLogLevel::LOG_ERROR);

        let args: Vec<_> = env::args().collect();
            if args.len() != 2 {
                println!("ERROR: invalid args!");
                process::exit(0);
            }
        
        let mut instance = AppManager {
            
            clock_timer: 0.,

            
            rl: raylib_handle,
            thread: raylib_thread,
            canvas: Image::gen_image_color(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, Color::GREENYELLOW),
            
            config: config::read_config(),
            theme_manager: ThemeManager::new(),
            input_manager: InputManager::new(),
            emulator: Emu::new(),

            args: args,
        };
        instance.input_manager.generate_keymaps_from_config(&instance.config);
        instance.theme_manager.parse_themes(&instance.config);

        instance.rl.set_target_fps(instance.config.max_fps);
        instance
    }

    pub fn main_loop(&mut self) {
        while !self.rl.window_should_close() {
            self.emulator.tick();
            self.update_clock();
            
            self.input_manager.handle_game_input(&mut self.emulator, &self.rl);
            let actions = self.input_manager.handle_emu_input(&self.rl);
            self.handle_action(actions);

            self.render_game();
        }
    }

    fn render_game(&mut self) {
        let screenbuffer = self.emulator.get_display();
        
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let pixel_color = if screenbuffer[x + SCREEN_WIDTH * y] {
                    self.get_ui_col("FG".to_string())
                } else {
                    self.get_ui_col("BG".to_string())
                };
                
                self.canvas.draw_pixel(x as i32, y as i32, pixel_color);
            }
        }

        let texture = self.rl.load_texture_from_image(&self.thread, &self.canvas).unwrap();
        let rom_path = self.args[1].clone();
        let text_col = self.get_ui_col("FILE".to_string());
        
        let mut d = self.rl.begin_drawing(&self.thread);
        
        d.clear_background(Color::RED);
        
        d.draw_texture_ex(&texture, Vector2::new(0.,0.), 0., (WIN_SCALE_FAC) as f32, Color::WHITE);


        let text_width = d.measure_text(&rom_path.as_str(), 20) as u32;
        let x_pos = ((WIN_WIDTH - text_width) as f32) * 0.5;
        if self.config.show_path {
            d.draw_text(&self.args[1], x_pos as i32, 12, 20, text_col);
        }
        if self.config.show_fps {
            d.draw_text(&format!("{}", d.get_fps()), 10, 10, 20, text_col);
        }
    }

    fn update_clock(&mut self) {
        self.clock_timer += self.rl.get_frame_time();
    
        if self.clock_timer >= 1. / (self.config.tps as f32) {
            self.emulator.tick_timers();
            self.clock_timer = 0.;
        }
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

    fn handle_action(&mut self, actions: Vec<String>) {
        for action in actions {
            match action.as_str() {
                "RESET" => {
                    println!("ACTION: RESET EMULATOR");
                    self.emulator.reset();
                    self.load_rom(None);
                },
                "NEXT_THEME" => {
                    println!("ACTION: Switched theme");
                    self.next_theme();
                },
                _ => {
                    print!("ERROR: Unimplemented ACTION");
                }
            }
        }
    }

    fn get_ui_col(&self, color_name: String) -> Color {
        let theme: HashMap<String,String> = self.theme_manager.themes[self.theme_manager.theme_index as usize].clone();

        let color_code = theme.get(&color_name).expect("ERROR: Invalid theme color key used").replace("#", "");

        Color::from_hex(&color_code).expect("ERROR: invalid theme data")
    }

    fn next_theme(&mut self) {
        if self.theme_manager.theme_index < (self.theme_manager.num_themes - 1) {
            self.theme_manager.theme_index += 1;
        }
        else {
            self.theme_manager.theme_index = 0;
        }

    }
}

