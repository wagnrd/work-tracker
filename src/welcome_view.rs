use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct WelcomeView;

pub struct WelcomeViewWidgets;

impl SimpleComponent for WelcomeView {
    type Input = ();
    type Output = ();
    type Init = ();
    type Root = gtk::CenterBox;
    type Widgets = WelcomeViewWidgets;

    fn init_root() -> Self::Root {
        gtk::CenterBox::builder()
            .hexpand(true)
            .vexpand(true)
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build()
    }

    fn init(
        _args: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let start_button = adw::SplitButton::builder().label("Start work").build();

        root.set_center_widget(Some(&start_button));

        let model = WelcomeView;
        let widgets = WelcomeViewWidgets;

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {}

    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}
