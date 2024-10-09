mod ui;
mod config;
mod themes;

use ui::UI;

fn main() {
    let mut ui = UI::new();

    ui.load_rom(None);

    ui.main_loop();
}
