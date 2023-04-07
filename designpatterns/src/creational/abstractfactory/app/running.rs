use super::render::render;
use crate::creational::abstractfactory::macosgui::factory::MacFactory;
use crate::creational::abstractfactory::windowsgui::factory::WindowsFactory;

pub fn running_abstract() {
    let windows = true;

    if windows {
        render(WindowsFactory);
    } else {
        render(MacFactory);
    }
}
