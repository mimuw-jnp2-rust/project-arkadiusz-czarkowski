use bevy::prelude::*;

use crate::common::*;
use crate::game_state::*;
use crate::game_textures::*;
use crate::spawn_tile::*;

pub fn mouse_pressed_system(
    buttons: Res<Input<MouseButton>>,
    mpos: Res<MousePosition>,
    mut sel: ResMut<SelectedSquare>,
    query: Query<(Entity, &mut Position, &mut Transform, &mut Handle<Image>)>,
    query_highlight: Query<Entity, With<Highlight>>,
    game_textures: Res<GameTextures>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = mpos.position {
            if let Some(sel_pos) = sel.position {
                delete_highlight(&mut commands, &query_highlight);
                sel.position = None;
                game_state.player_move(game_textures, &mut commands, query, sel_pos, position);
            } else {
                sel.position = mpos.position;
                spawn_tile(
                    &mut commands,
                    game_textures.highlight.clone(),
                    position,
                    true,
                );
            }
        } else {
            sel.position = None;
            delete_highlight(&mut commands, &query_highlight);
        }
    }
    if buttons.just_pressed(MouseButton::Right) {
        sel.position = None;
        delete_highlight(&mut commands, &query_highlight);
    }
}
