use crate::creational::abstractfactory::gui::button::Button;

pub struct WindowButton;

impl Button for WindowButton {
    fn press(&self) {
        println!("Windows button has pressed")
    }
}
