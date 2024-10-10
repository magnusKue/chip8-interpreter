mod app;
mod config;
mod themes;
mod input;
mod audio;

use app::AppManager;

fn main() {
    let mut ui = AppManager::new();

    ui.load_rom(None);

    ui.main_loop();
}
