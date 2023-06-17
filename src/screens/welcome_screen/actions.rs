use relm4::actions::{ActionGroupName, ActionName};

pub struct StartButtonActionGroupName;

impl ActionGroupName for StartButtonActionGroupName {
    const NAME: &'static str = "start_button";
}

pub struct CustomTimeActionName;

impl ActionName for CustomTimeActionName {
    type Group = StartButtonActionGroupName;
    type Target = ();
    type State = ();
    const NAME: &'static str = "custom_time";
}

pub struct NowActionName;

impl ActionName for NowActionName {
    type Group = StartButtonActionGroupName;
    type Target = ();
    type State = ();
    const NAME: &'static str = "now";
}
