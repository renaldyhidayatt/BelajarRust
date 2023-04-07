use crate::creational::abstractfactory::gui::button::Button;

pub struct MacButton;

impl Button for MacButton {
    fn press(&self) {
        println!("MacOs button has pressed")
    }
}
