use crate::creational::abstractfactory::gui::button::CheckBox;

pub struct MacCheckbox;

impl CheckBox for MacCheckbox {
    fn switch(&self) {
        println!("MacOS checkbox has switched");
    }
}
