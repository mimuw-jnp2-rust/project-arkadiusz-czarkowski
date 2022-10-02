use crate::common::*;
use crate::game_state::*;
use crate::game_textures::*;
use crate::piece::*;
use bevy::prelude::*;

pub fn spawn_piece(
    commands: &mut Commands,
    texture: Handle<Image>,
    position: Position,
    piece_color: PieceColor,
    piece_type: PieceType,
    game_state: &mut ResMut<GameState>,
) {
    let mut transform = Transform {
        translation: real_piece_position(position),
        ..Default::default()
    };
    transform.scale *= SCALING_FACTOR;

    commands
        .spawn_bundle(SpriteBundle {
            texture,
            transform,
            ..Default::default()
        })
        .insert(position);

    game_state.board[position.0 as usize][position.1 as usize] = Some(Piece {
        piece_color,
        piece_type,
        x: position.0,
        y: position.1,
    });
}
