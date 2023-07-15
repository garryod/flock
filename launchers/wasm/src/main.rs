use bevy::prelude::*;
use my_game::LAUNCHER_TITLE;

fn set_window_title(title: &str) {
    web_sys::window()
        .and_then(|w| w.document())
        .expect("Unable to get DOM")
        .set_title(title);
}

fn main() {
    // Start the Bevy App
    set_window_title(LAUNCHER_TITLE);
    let mut app = my_game::app();
    info!("Starting launcher: WASM");
    app.run();
}
