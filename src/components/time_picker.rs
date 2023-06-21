use crate::components::counter::{CounterInit, CounterOutput};
use crate::components::Counter;
use chrono::Timelike;
use relm4::gtk::prelude::*;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

pub struct TimePicker {
    time: chrono::DateTime<chrono::Local>,
}

pub struct TimePickerWidgets {
    hour_counter: Controller<Counter>,
    minute_counter: Controller<Counter>,
}

#[derive(Debug)]
pub enum TimePickerInput {
    HourChanged(u32),
    MinuteChanged(u32),
}

#[derive(Debug)]
pub enum TimePickerOutput {}

impl SimpleComponent for TimePicker {
    type Input = TimePickerInput;
    type Output = TimePickerOutput;
    type Init = ();
    type Root = gtk::Box;
    type Widgets = TimePickerWidgets;

    fn init_root() -> Self::Root {
        gtk::Box::builder()
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build()
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let current_time = chrono::Local::now();

        // Hour column
        let hour_counter: Controller<Counter> = Counter::builder()
            .launch(CounterInit {
                initial_value: current_time.hour() as i32,
                max_value: 23,
                min_value: 0,
                looping: true,
            })
            .forward(sender.input_sender(), |message| match message {
                CounterOutput::ValueChanged(value) => TimePickerInput::HourChanged(value as u32),
            });
        root.append(hour_counter.widget());

        // Middle colon column
        let colon_label = gtk::Text::builder().text(":").build();
        root.append(&colon_label);

        // Minute column
        let minute_counter: Controller<Counter> = Counter::builder()
            .launch(CounterInit {
                initial_value: current_time.minute() as i32,
                max_value: 59,
                min_value: 0,
                looping: true,
            })
            .forward(sender.input_sender(), |message| match message {
                CounterOutput::ValueChanged(value) => TimePickerInput::MinuteChanged(value as u32),
            });
        root.append(minute_counter.widget());

        let model = TimePicker { time: current_time };
        let widgets = TimePickerWidgets {
            hour_counter,
            minute_counter,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            TimePickerInput::HourChanged(hour) => self.time = self.time.with_hour(hour).unwrap(),
            TimePickerInput::MinuteChanged(hour) => {
                self.time = self.time.with_minute(hour).unwrap()
            }
        }
    }
}
