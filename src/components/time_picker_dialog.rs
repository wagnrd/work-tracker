use crate::components::time_picker::{TimePicker, TimePickerInit, TimePickerOutput};
use relm4::adw::glib::clone;
use relm4::adw::prelude::AdwWindowExt;
use relm4::gtk::prelude::{BoxExt, ButtonExt, WidgetExt};
use relm4::{
    adw, gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

pub struct TimePickerDialog {
    time: chrono::DateTime<chrono::Local>,
}

pub struct TimePickerDialogInit {
    pub initial_time: chrono::DateTime<chrono::Local>,
}

#[derive(Debug)]
pub enum TimePickerDialogInput {
    TimeChanged(chrono::DateTime<chrono::Local>),
    Confirm,
    Cancel,
}

#[derive(Debug)]
pub enum TimePickerDialogOutput {
    Confirmed(chrono::DateTime<chrono::Local>),
    Canceled,
}

pub struct TimePickerDialogWidgets {
    time_picker: Controller<TimePicker>,
}

impl SimpleComponent for TimePickerDialog {
    type Input = TimePickerDialogInput;
    type Output = TimePickerDialogOutput;
    type Init = TimePickerDialogInit;
    type Root = adw::Window;
    type Widgets = TimePickerDialogWidgets;

    fn init_root() -> Self::Root {
        adw::Window::builder()
            .application(&relm4::main_adw_application())
            .modal(true)
            .title("")
            .default_width(60)
            .build()
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let cancel_button = gtk::Button::builder().label("Cancel").build();
        cancel_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(TimePickerDialogInput::Cancel);
        }));

        let confirm_button = gtk::Button::builder()
            .label("Confirm")
            .css_classes(["suggested-action"])
            .build();
        confirm_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(TimePickerDialogInput::Confirm);
        }));

        let header_bar = adw::HeaderBar::builder()
            .show_end_title_buttons(false)
            .show_start_title_buttons(false)
            .build();
        header_bar.pack_start(&cancel_button);
        header_bar.pack_end(&confirm_button);

        let time_picker = TimePicker::builder()
            .launch(TimePickerInit {
                initial_time: init.initial_time,
            })
            .forward(sender.input_sender(), |message| match message {
                TimePickerOutput::TimeChanged(time) => TimePickerDialogInput::TimeChanged(time),
            });
        time_picker.widget().set_margin_top(20);
        time_picker.widget().set_margin_bottom(20);
        time_picker.widget().set_margin_start(75);
        time_picker.widget().set_margin_end(75);

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();
        vbox.append(&header_bar);
        vbox.append(time_picker.widget());
        root.set_content(Some(&vbox));

        let model = TimePickerDialog {
            time: init.initial_time,
        };
        let widgets = TimePickerDialogWidgets { time_picker };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            TimePickerDialogInput::TimeChanged(time) => self.time = time,
            TimePickerDialogInput::Cancel => {
                sender.output(TimePickerDialogOutput::Canceled).unwrap()
            }
            TimePickerDialogInput::Confirm => sender
                .output(TimePickerDialogOutput::Confirmed(self.time))
                .unwrap(),
        }
    }
}
