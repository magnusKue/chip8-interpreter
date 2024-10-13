mod app;
mod audio;

mod savestates;
mod config;

mod input;
mod themes;
mod graphics;

use app::AppManager;

fn main() {
    let mut ui = AppManager::new();

    ui.load_rom(None);

    ui.main_loop();
}
