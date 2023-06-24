use crate::screens::WelcomeScreen;
use relm4::adw::prelude::*;
use relm4::{
    adw, gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};
use std::convert::identity;

pub struct App;

pub struct AppWidgets {
    screen: Controller<WelcomeScreen>,
}

impl SimpleComponent for App {
    type Input = ();
    type Output = ();
    type Init = ();
    type Root = adw::Window;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        adw::Window::builder()
            .title("Login Client")
            .application(&relm4::main_adw_application())
            .default_width(300)
            .default_height(200)
            .build()
    }

    fn init(
        _args: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        root.set_content(Some(&vbox));
        vbox.append(&adw::HeaderBar::default());

        let welcome_screen: Controller<WelcomeScreen> = WelcomeScreen::builder()
            .launch(())
            .forward(sender.input_sender(), identity);
        vbox.append(welcome_screen.widget());

        let model = App {};
        let widgets = AppWidgets {
            screen: welcome_screen,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {}

    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}
