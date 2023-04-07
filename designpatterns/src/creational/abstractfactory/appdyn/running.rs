use crate::creational::abstractfactory::gui::button::GuiFactoryDynamic;

use super::render::render;
use crate::creational::abstractfactory::macosgui::factory::MacFactory;
use crate::creational::abstractfactory::windowsgui::factory::WindowsFactory;

pub fn running_app() {
    let windows = false;

    let factory: &dyn GuiFactoryDynamic = if windows {
        &WindowsFactory
    } else {
        &MacFactory
    };

    let button = factory.create_button();
    button.press();

    render(factory);
}
