use crate::screens::welcome_screen::actions::{
    CustomTimeActionName, NowActionName, StartButtonActionGroupName,
};
use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::gtk::gio;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

pub struct WelcomeScreen;

pub struct WelcomeScreenWidgets;

impl SimpleComponent for WelcomeScreen {
    type Input = ();
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
        _args: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let start_button = adw::SplitButton::builder().label("Start work").build();
        root.set_center_widget(Some(&start_button));

        let custom_time_menu_item = RelmAction::<CustomTimeActionName>::to_menu_item("Custom time");
        let now_menu_item = RelmAction::<NowActionName>::to_menu_item("Now");
        let menu = gio::Menu::default();
        menu.append_item(&custom_time_menu_item);
        menu.append_item(&now_menu_item);
        start_button.set_menu_model(Some(&menu));

        let mut action_group = RelmActionGroup::<StartButtonActionGroupName>::new();
        let custom_time_action = RelmAction::<CustomTimeActionName>::new_stateless(|_| {
            println!("Custom Time Action");
        });
        let now_action = RelmAction::<NowActionName>::new_stateless(|_| {
            println!("Now Action");
        });
        action_group.add_action(custom_time_action);
        action_group.add_action(now_action);
        action_group.register_for_widget(start_button);

        let model = WelcomeScreen;
        let widgets = WelcomeScreenWidgets;

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {}

    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}
