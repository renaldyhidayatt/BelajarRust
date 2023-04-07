use super::init::initialize;

pub fn running() {
    let dialog = initialize();

    dialog.render();
    dialog.refresh();
}
