use crate::components::TimePicker;
use crate::screens::welcome_screen::actions::{
    CustomTimeActionName, NowActionName, StartButtonActionGroupName,
};
use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::adw::glib::clone;
use relm4::gtk::gio;
use relm4::gtk::prelude::BoxExt;
use relm4::{
    adw, gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};
use std::time;

pub struct WelcomeScreen {
    custom_start_time: Option<time::Instant>,
    is_time_selector_visible: bool,
}

#[derive(Debug)]
pub enum WelcomeScreenInput {
    StartTimeNow,
    CustomStartTime,
}

pub struct WelcomeScreenWidgets {
    time_picker: Controller<TimePicker>,
}

impl SimpleComponent for WelcomeScreen {
    type Input = WelcomeScreenInput;
    type Output = ();
    type Init = ();
    type Root = gtk::CenterBox;
    type Widgets = WelcomeScreenWidgets;

    fn init_root() -> Self::Root {
        gtk::CenterBox::builder()
            .hexpand(true)
            .vexpand(true)
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build()
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        root.set_center_widget(Some(&vbox));

        let start_button = adw::SplitButton::builder().label("Start work").build();
        vbox.append(&start_button);

        let custom_time_menu_item = RelmAction::<CustomTimeActionName>::to_menu_item("Custom time");
        let now_menu_item = RelmAction::<NowActionName>::to_menu_item("Now");
        let menu = gio::Menu::default();
        menu.append_item(&custom_time_menu_item);
        menu.append_item(&now_menu_item);
        start_button.set_menu_model(Some(&menu));

        let mut action_group = RelmActionGroup::<StartButtonActionGroupName>::new();
        let custom_time_action =
            RelmAction::<CustomTimeActionName>::new_stateless(clone!(@strong sender => move |_| {
                sender.input(WelcomeScreenInput::CustomStartTime)
            }));
        let now_action =
            RelmAction::<NowActionName>::new_stateless(clone!(@strong sender => move |_| {
                sender.input(WelcomeScreenInput::StartTimeNow)
            }));
        action_group.add_action(custom_time_action);
        action_group.add_action(now_action);
        action_group.register_for_widget(start_button);

        let time_picker: Controller<TimePicker> = TimePicker::builder()
            .launch(())
            .forward(sender.input_sender(), |message| match message {});
        vbox.append(time_picker.widget());

        let model = WelcomeScreen {
            is_time_selector_visible: false,
            custom_start_time: None,
        };
        let widgets = WelcomeScreenWidgets { time_picker };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            WelcomeScreenInput::StartTimeNow => {
                self.custom_start_time = None;
                println!("start time now");
            }
            WelcomeScreenInput::CustomStartTime => self.is_time_selector_visible = true,
        }
    }

    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}
