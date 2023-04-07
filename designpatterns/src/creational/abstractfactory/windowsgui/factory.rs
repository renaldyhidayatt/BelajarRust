use crate::creational::abstractfactory::gui::button::{
    Button, CheckBox, GuiFactory, GuiFactoryDynamic,
};

use super::{button::WindowButton, checkbox::WindowsCheckbox};

pub struct WindowsFactory;

impl GuiFactory for WindowsFactory {
    type B = WindowButton;
    type C = WindowsCheckbox;

    fn create_button(&self) -> Self::B {
        WindowButton
    }

    fn create_checkbox(&self) -> Self::C {
        WindowsCheckbox
    }
}

impl GuiFactoryDynamic for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowButton)
    }

    fn create_checkbox(&self) -> Box<dyn CheckBox> {
        Box::new(WindowsCheckbox)
    }
}
