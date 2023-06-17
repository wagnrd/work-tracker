use relm4::actions::{ActionGroupName, ActionName};

pub struct WelcomeScreenActionGroupName;

impl ActionGroupName for WelcomeScreenActionGroupName {
    const NAME: &'static str = "welcome_screen";
}

pub struct CustomTimeActionName;

impl ActionName for CustomTimeActionName {
    type Group = WelcomeScreenActionGroupName;
    type Target = ();
    type State = ();
    const NAME: &'static str = "custom_time";
}

pub struct NowActionName;

impl ActionName for NowActionName {
    type Group = WelcomeScreenActionGroupName;
    type Target = ();
    type State = ();
    const NAME: &'static str = "now";
}
