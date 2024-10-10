mod app;
mod audio;
mod config;
mod input;
mod themes;

use app::AppManager;

fn main() {
    let mut ui = AppManager::new();

    ui.load_rom(None);

    ui.main_loop();
}
