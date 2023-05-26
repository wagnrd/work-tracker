use relm4::RelmApp;

use crate::application::App;

mod application;

fn main() {
    let app = RelmApp::new("de.wagnrd.login-client");
    app.run::<App>(());
}
