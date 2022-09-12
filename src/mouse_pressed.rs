use crate::common::*;
use crate::game_state::*;
use crate::game_textures::*;
use crate::spawn_tile::*;
use bevy::prelude::*;

pub fn mouse_pressed_system(
    buttons: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
    mut selected_square: ResMut<SelectedSquare>,
    query: Query<(Entity, &mut Position, &mut Transform, &mut Handle<Image>)>,
    query_highlight: Query<Entity, With<Highlight>>,
    game_textures: Res<GameTextures>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = mouse_position.position {
            if let Some(selected_square_position) = selected_square.position {
                delete_highlight(&mut commands, &query_highlight);
                selected_square.position = None;
                game_state.player_move(
                    game_textures,
                    &mut commands,
                    query,
                    selected_square_position,
                    position,
                );
            } else {
                selected_square.position = mouse_position.position;
                spawn_tile(
                    &mut commands,
                    game_textures.highlight.clone(),
                    position,
                    true,
                );
            }
        } else {
            selected_square.position = None;
            delete_highlight(&mut commands, &query_highlight);
        }
    }
    if buttons.just_pressed(MouseButton::Right) {
        selected_square.position = None;
        delete_highlight(&mut commands, &query_highlight);
    }
}
