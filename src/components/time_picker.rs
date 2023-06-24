use crate::components::counter::{Counter, CounterInit, CounterOutput};
use chrono::Timelike;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

pub struct TimePicker {
    time: chrono::DateTime<chrono::Local>,
}

pub struct TimePickerInit {
    pub initial_time: chrono::DateTime<chrono::Local>,
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
pub enum TimePickerOutput {
    TimeChanged(chrono::DateTime<chrono::Local>),
}

impl SimpleComponent for TimePicker {
    type Input = TimePickerInput;
    type Output = TimePickerOutput;
    type Init = TimePickerInit;
    type Root = gtk::CenterBox;
    type Widgets = TimePickerWidgets;

    fn init_root() -> Self::Root {
        gtk::CenterBox::builder()
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build()
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Hour column
        let hour_counter: Controller<Counter> = Counter::builder()
            .launch(CounterInit {
                initial_value: init.initial_time.hour() as i32,
                max_value: 23,
                min_value: 0,
                looping: true,
            })
            .forward(sender.input_sender(), |message| match message {
                CounterOutput::ValueChanged(value) => TimePickerInput::HourChanged(value as u32),
            });
        root.set_start_widget(Some(hour_counter.widget()));

        // Middle colon column
        let middle_colon = gtk::Label::builder()
            .label(":")
            .margin_start(10)
            .margin_end(10)
            .build();
        root.set_center_widget(Some(&middle_colon));

        // Minute column
        let minute_counter: Controller<Counter> = Counter::builder()
            .launch(CounterInit {
                initial_value: init.initial_time.minute() as i32,
                max_value: 59,
                min_value: 0,
                looping: true,
            })
            .forward(sender.input_sender(), |message| match message {
                CounterOutput::ValueChanged(value) => TimePickerInput::MinuteChanged(value as u32),
            });
        root.set_end_widget(Some(minute_counter.widget()));

        let model = TimePicker {
            time: init.initial_time,
        };
        let widgets = TimePickerWidgets {
            hour_counter,
            minute_counter,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            TimePickerInput::HourChanged(hour) => self.time = self.time.with_hour(hour).unwrap(),
            TimePickerInput::MinuteChanged(hour) => {
                self.time = self.time.with_minute(hour).unwrap()
            }
        }

        sender
            .output(TimePickerOutput::TimeChanged(self.time))
            .unwrap();
    }
}
