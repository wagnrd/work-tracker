mod app;
mod screens;
mod components;

use crate::app::App;
use relm4::RelmApp;

fn main() {
    let app = RelmApp::new("de.wagnrd.work-tracker");
    app.run::<App>(());
}
