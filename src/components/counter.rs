use relm4::gtk::glib::clone;
use relm4::gtk::prelude::{BoxExt, ButtonExt, EditableExt};
use relm4::{gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct Counter {
    value: i32,
    min_value: i32,
    max_value: i32,
}

pub struct CounterInit {
    pub max_value: i32,
    pub min_value: i32,
    pub initial_value: i32,
}

pub struct CounterWidgets {
    entry: gtk::Entry,
}

#[derive(Debug)]
pub enum CounterInput {
    Increase,
    Decrease,
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
        let up_button = gtk::Button::builder().icon_name("go-up-symbolic").build();
        up_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(CounterInput::Increase)
        }));
        root.append(&up_button);
        let entry = gtk::Entry::builder()
            .text(init.initial_value.to_string())
            .build();
        root.append(&entry);
        let down_button = gtk::Button::builder().icon_name("go-down-symbolic").build();
        down_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(CounterInput::Decrease)
        }));
        root.append(&down_button);

        let model = Counter {
            value: init.initial_value,
            max_value: init.max_value,
            min_value: init.min_value,
        };
        let widgets = CounterWidgets { entry };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            CounterInput::Increase => {
                if self.value < self.max_value {
                    self.value += 1;
                }
            }
            CounterInput::Decrease => {
                if self.value > self.min_value {
                    self.value -= 1
                }
            }
        }

        sender
            .output(CounterOutput::ValueChanged(self.value))
            .unwrap();
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        widgets.entry.set_text(self.value.to_string().as_str());
    }
}
