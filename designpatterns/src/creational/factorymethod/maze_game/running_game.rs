use super::game;
use super::magic_maze::MagicMaze;
use super::ordinary_maze::OrdinaryMaze;

pub fn running_game() {
    let ordinary_maze = OrdinaryMaze::new();

    game::run(ordinary_maze);

    let magic_maze = MagicMaze::new();

    game::run(magic_maze);
}
