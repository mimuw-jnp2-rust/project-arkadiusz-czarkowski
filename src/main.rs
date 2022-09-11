use bevy::prelude::*;
use std::env;

mod common;
mod computer_moves;
mod create_physical_board;
mod cursor;
mod game_state;
mod game_textures;
mod mouse_pressed;
mod physical_board;
mod piece;
mod piece_square_tables;
mod program_options;
mod setup;
mod spawn_piece;
mod spawn_tile;

fn main() {
    program_options::program_options(env::args().collect::<Vec<String>>());
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup::setup)
        .add_startup_system_to_stage(
            StartupStage::PostStartup,
            create_physical_board::create_board,
        )
        .add_startup_system_to_stage(
            StartupStage::PostStartup,
            create_physical_board::create_pieces,
        )
        .add_system(cursor::cursor_position_system)
        .add_system(mouse_pressed::mouse_pressed_system)
        .add_system(computer_moves::computer_moves_system)
        .run();
}
