
use relm4::{adw, ComponentParts, ComponentSender, gtk, SimpleComponent};
use relm4::adw::prelude::*;

pub struct App;

#[derive(Debug)]
pub enum AppMsg {}

pub struct AppWidgets {}

impl SimpleComponent for App {
    type Input = AppMsg;
    type Output = ();
    type Init = ();
    type Root = adw::Window;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        adw::Window::builder()
            .title("Login Client")
            .default_width(300)
            .default_height(200)
            .build()
    }

    fn init(
        _args: Self::Init,
        window: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        vbox.append(&adw::HeaderBar::default());

        window.set_content(Some(&vbox));

        let model = App {};
        let widgets = AppWidgets {};

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {}

    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}