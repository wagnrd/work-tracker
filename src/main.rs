mod app;
mod screens;

use crate::app::App;
use relm4::RelmApp;

fn main() {
    let app = RelmApp::new("de.wagnrd.work-tracker");
    app.run::<App>(());
}
