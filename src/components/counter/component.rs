use crate::components::counter::service;
use relm4::gtk::glib::clone;
use relm4::gtk::prelude::*;
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct Counter {
    value: i32,
    min_value: i32,
    max_value: i32,
    looping: bool,
    is_internal_update: bool,
}

pub struct CounterInit {
    pub max_value: i32,
    pub min_value: i32,
    pub initial_value: i32,
    pub looping: bool,
}

pub struct CounterWidgets {
    entry: gtk::Entry,
}

#[derive(Debug)]
pub enum CounterInput {
    Increase,
    Decrease,
    EntryChanged(String),
}

#[derive(Debug)]
pub enum CounterOutput {
    ValueChanged(i32),
}

impl SimpleComponent for Counter {
    type Input = CounterInput;
    type Output = CounterOutput;
    type Init = CounterInit;
    type Root = gtk::Box;
    type Widgets = CounterWidgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build()
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let up_button = gtk::Button::builder()
            .icon_name("go-up-symbolic")
            .css_classes(["circular"])
            .build();
        up_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(CounterInput::Increase);
        }));
        root.append(&up_button);

        let entry = gtk::Entry::builder()
            .text(init.initial_value.to_string())
            .build();
        entry.connect_changed(clone!(@strong sender => move |entry| {
            sender.input(CounterInput::EntryChanged(entry.text().to_string()));
        }));
        root.append(&entry);

        let down_button = gtk::Button::builder()
            .icon_name("go-down-symbolic")
            .css_classes(["circular"])
            .build();
        down_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(CounterInput::Decrease);
        }));
        root.append(&down_button);

        let model = Counter {
            value: init.initial_value,
            max_value: init.max_value,
            min_value: init.min_value,
            looping: init.looping,
            is_internal_update: false,
        };
        let widgets = CounterWidgets { entry };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            CounterInput::Increase => {
                if self.value < self.max_value {
                    self.value += 1;
                } else if self.looping {
                    self.value = self.min_value;
                }

                self.is_internal_update = true;
            }
            CounterInput::Decrease => {
                if self.value > self.min_value {
                    self.value -= 1;
                } else if self.looping {
                    self.value = self.max_value;
                }

                self.is_internal_update = true;
            }
            CounterInput::EntryChanged(value) => {
                if !self.is_internal_update {
                    let unclamped_value =
                        service::filter_integers_from_text(value).unwrap_or(self.value);
                    self.value = num::clamp(unclamped_value, self.min_value, self.max_value);
                    self.is_internal_update = true;

                    sender
                        .output(CounterOutput::ValueChanged(self.value))
                        .unwrap();
                } else {
                    self.is_internal_update = false;
                }
            }
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        if self.value.to_string() != widgets.entry.text() {
            widgets.entry.set_text(self.value.to_string().as_str());
        }
    }
}
