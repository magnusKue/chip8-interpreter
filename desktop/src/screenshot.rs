use chip8_core::Emu;

use chrono::Utc;
use std::ffi::CString;
use raylib::ffi::TakeScreenshot;
use crate::config::Config;


pub fn take_screenshot(config: &Config, emulator: &Emu) {
    let path = create_path(config);
    println!("{}",path);
    if config.capture_ui {
        println!("INFO: Capturing UI!");
        // save whole window
        let c_path = CString::new(path).expect("ERROR: Creating cString from path failed");
        unsafe {
            TakeScreenshot(c_path.as_ptr());
        }
    }   
    else {
        // save screenbuffer only
    }
}


pub fn create_path(config: &Config) -> String {
    let now = Utc::now();
    let ss_title = now.format("%Y-%m-%d_at_%H:%M:%S").to_string() + &config.screenshot_format;
    ss_title
}
