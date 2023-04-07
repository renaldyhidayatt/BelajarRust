use crate::creational::abstractfactory::gui::button::GuiFactory;

pub fn render(factory: impl GuiFactory) {
    let button1 = factory.create_button();
    let button2 = factory.create_button();
    let checkbox1 = factory.create_checkbox();
    let checkbox2 = factory.create_checkbox();

    use crate::creational::abstractfactory::gui::button::{Button, CheckBox};

    button1.press();
    button2.press();
    checkbox1.switch();
    checkbox2.switch();
}
