use crate::creational::abstractfactory::gui::button::CheckBox;

pub struct WindowsCheckbox;

impl CheckBox for WindowsCheckbox {
    fn switch(&self) {
        println!("Windows checkbox has switched");
    }
}
