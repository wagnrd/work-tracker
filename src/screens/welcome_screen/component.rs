use crate::components::time_picker_dialog::{
    TimePickerDialog, TimePickerDialogInit, TimePickerDialogOutput,
};
use crate::screens::welcome_screen::actions::{
    CustomTimeActionName, NowActionName, StartButtonActionGroupName,
};
use chrono::Timelike;
use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::adw::glib::clone;
use relm4::adw::prelude::*;
use relm4::gtk::gio;
use relm4::{
    adw, gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

pub struct WelcomeScreen {
    custom_start_time: Option<chrono::DateTime<chrono::Local>>,
    is_time_picker_dialog_visible: bool,
}

#[derive(Debug)]
pub enum WelcomeScreenInput {
    StartTimeNowSelected,
    CustomStartTimeSelected(chrono::DateTime<chrono::Local>),
    ShowTimePickerDialog,
    HideTimePickerDialog,
}

pub struct WelcomeScreenWidgets {
    dialog: Controller<TimePickerDialog>,
    start_time_label: gtk::Label,
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
            .spacing(10)
            .build();
        root.set_center_widget(Some(&vbox));

        let start_button = adw::SplitButton::builder().label("Start work").build();
        vbox.append(&start_button);

        let start_time_label = gtk::Label::builder()
            .halign(gtk::Align::Center)
            .visible(false)
            .build();
        vbox.append(&start_time_label);

        // Button menu
        let custom_time_menu_item = RelmAction::<CustomTimeActionName>::to_menu_item("Custom time");
        let now_menu_item = RelmAction::<NowActionName>::to_menu_item("Now");
        let menu = gio::Menu::default();
        menu.append_item(&custom_time_menu_item);
        menu.append_item(&now_menu_item);
        start_button.set_menu_model(Some(&menu));

        let mut action_group = RelmActionGroup::<StartButtonActionGroupName>::new();
        let custom_time_action =
            RelmAction::<CustomTimeActionName>::new_stateless(clone!(@strong sender => move |_| {
                sender.input(WelcomeScreenInput::ShowTimePickerDialog)
            }));
        let now_action =
            RelmAction::<NowActionName>::new_stateless(clone!(@strong sender => move |_| {
                sender.input(WelcomeScreenInput::StartTimeNowSelected)
            }));
        action_group.add_action(custom_time_action);
        action_group.add_action(now_action);
        action_group.register_for_widget(start_button);

        // Time Picker Dialog
        let dialog = TimePickerDialog::builder()
            .launch(TimePickerDialogInit {
                initial_time: chrono::Local::now(),
            })
            .forward(sender.input_sender(), |message| match message {
                TimePickerDialogOutput::Confirmed(time) => {
                    WelcomeScreenInput::CustomStartTimeSelected(time)
                }
                TimePickerDialogOutput::Canceled => WelcomeScreenInput::HideTimePickerDialog,
            });

        let model = WelcomeScreen {
            is_time_picker_dialog_visible: false,
            custom_start_time: None,
        };
        let widgets = WelcomeScreenWidgets {
            dialog,
            start_time_label,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            WelcomeScreenInput::ShowTimePickerDialog => self.is_time_picker_dialog_visible = true,
            WelcomeScreenInput::HideTimePickerDialog => self.is_time_picker_dialog_visible = false,
            WelcomeScreenInput::StartTimeNowSelected => self.custom_start_time = None,
            WelcomeScreenInput::CustomStartTimeSelected(time) => {
                self.custom_start_time = Some(time);
                self.is_time_picker_dialog_visible = false;
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        if self.is_time_picker_dialog_visible {
            widgets.dialog.widget().set_visible(true);
        } else {
            widgets.dialog.widget().set_visible(false);
        }

        if let Some(custom_start_time) = self.custom_start_time {
            widgets.start_time_label.set_label(
                format!(
                    "From {}:{}",
                    custom_start_time.hour(),
                    custom_start_time.minute()
                )
                .as_str(),
            );
            widgets.start_time_label.set_visible(true);
        } else {
            widgets.start_time_label.set_visible(false);
        }
    }
}
