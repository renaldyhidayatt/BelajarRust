mod creational;
mod structural;

use crate::creational::factorymethod::dialog_rendering::running_rending;
use crate::creational::factorymethod::maze_game::running_game;
use crate::structural::composite::running;

fn main() {
    running_game::running_game();
    running_rending::running();

    running::running();
}
