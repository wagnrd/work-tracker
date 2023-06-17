use relm4::actions::{ActionGroupName, ActionName, RelmAction, RelmActionGroup};
use relm4::adw::prelude::*;
use relm4::gtk::gio;
use relm4::gtk::prelude::*;
use relm4::gtk::InputPurpose::Name;
use relm4::{adw, gtk, ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent, WidgetRef};

struct TestActionGroupName;

impl ActionGroupName for TestActionGroupName {
    const NAME: &'static str = "testgroup";
}

struct TestActionName;

impl ActionName for TestActionName {
    type Group = TestActionGroupName;
    type Target = ();
    type State = ();
    const NAME: &'static str = "test";
}

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

        let mut action_group = RelmActionGroup::<TestActionGroupName>::new();
        let action = RelmAction::<TestActionName>::new_stateless(|_| {
            println!("It worked!!");
        });
        action_group.add_action(action);
        action_group.register_for_main_application();

        let menu_item = RelmAction::<TestActionName>::to_menu_item("Test");
        menu_item.set_detailed_action("app.test");
        let menu = gio::Menu::default();
        menu.append_item(&menu_item);
        start_button.set_menu_model(Some(&menu));

        let model = WelcomeView;
        let widgets = WelcomeViewWidgets;

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {}

    fn update_view(&self, _widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {}
}
