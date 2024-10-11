use raylib::prelude::*;
use chip8_core::*;
use std::collections::HashMap;

use crate::{config::Config, themes::ThemeManager};

const WIN_SCALE_FAC: u32 = 15;
const WIN_WIDTH: u32 = (SCREEN_WIDTH as u32) * WIN_SCALE_FAC;
const WIN_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * WIN_SCALE_FAC;

pub struct GraphicsManager {  
    pub rl: RaylibHandle,
    pub thread: RaylibThread,
    pub canvas: Image,

    pub theme_manager: ThemeManager,
}

impl GraphicsManager {
    pub fn new() -> Self {
            
        let (mut raylib_handle, raylib_thread) = raylib::init()
            .resizable()
            .size(WIN_WIDTH as i32, WIN_HEIGHT as i32)
            .title("Chip8 emulator")
            .build();

        raylib_handle.set_trace_log(TraceLogLevel::LOG_NONE);
        raylib_handle.set_exit_key(None);

        GraphicsManager {
            rl: raylib_handle,
            thread: raylib_thread,
            canvas: Image::gen_image_color(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, Color::GREENYELLOW),
            
            theme_manager: ThemeManager::new(),
        }
    }


    pub fn render_game(&mut self, args: &[String], config: &Config, emulator: &Emu, framebuffer_modified: bool) {
        let texture = self.rl.load_texture_from_image(&self.thread, &self.canvas).unwrap();

        if framebuffer_modified {
            let screenbuffer = emulator.get_display();
            
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
        }

        

        let rom_path = args[1].clone();
        let text_col = self.get_ui_col("TEXT".to_string());

        let mut d = self.rl.begin_drawing(&self.thread);
        
        d.clear_background(Color::RED);

        d.draw_texture_ex(&texture, Vector2::new(0.,0.), 0., (WIN_SCALE_FAC) as f32, Color::WHITE);
        

        let text_width = d.measure_text(rom_path.as_str(), 20) as u32;
        let x_pos = ((WIN_WIDTH - text_width) as f32) * 0.5;
        if config.show_path {
            d.draw_text(&args[1], x_pos as i32, 12, 20, text_col);
        }
        if config.show_fps {
            d.draw_text(&format!("{}", d.get_fps()), 10, 10, 20, text_col);
        }
    }
    

    pub fn get_ui_col(&self, color_name: String) -> Color {
        let theme: HashMap<String,String> = self.theme_manager.themes[self.theme_manager.theme_index as usize].clone();

        let color_code = theme.get(&color_name).expect("ERROR: Invalid theme color key used").replace("#", "");

        Color::from_hex(&color_code).expect("ERROR: invalid theme data")
    }

    pub fn next_theme(&mut self) {
        if self.theme_manager.theme_index < (self.theme_manager.num_themes - 1) {
            self.theme_manager.theme_index += 1;
        }
        else {
            self.theme_manager.theme_index = 0;
        }

    }
}
