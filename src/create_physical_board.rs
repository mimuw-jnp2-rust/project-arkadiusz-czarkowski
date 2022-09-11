use crate::common::*;
use crate::game_state::*;
use crate::game_textures::*;
use crate::piece::*;
use crate::spawn_piece::*;
use crate::spawn_tile::*;
use bevy::prelude::*;

pub fn create_board(mut commands: Commands, game_textures: Res<GameTextures>) {
    let tilel = game_textures.tilel.clone();
    let tiled = game_textures.tiled.clone();
    for i in 0..8 {
        for j in 0..8 {
            let tile = if (i + j) % 2 == 0 { &tiled } else { &tilel };
            spawn_tile(&mut commands, tile.clone(), Position(i, j), false);
        }
    }
}

pub fn create_pieces(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut game_state: ResMut<GameState>,
) {
    spawn_piece(
        &mut commands,
        game_textures.kingl.clone(),
        Position(4, 0),
        PieceColor::White,
        PieceType::King,
        &mut game_state,
    );
    spawn_piece(
        &mut commands,
        game_textures.queenl.clone(),
        Position(3, 0),
        PieceColor::White,
        PieceType::Queen,
        &mut game_state,
    );
    for i in &[0, 7] {
        spawn_piece(
            &mut commands,
            game_textures.rookl.clone(),
            Position(*i, 0),
            PieceColor::White,
            PieceType::Rook,
            &mut game_state,
        );
    }
    for i in &[2, 5] {
        spawn_piece(
            &mut commands,
            game_textures.bishopl.clone(),
            Position(*i, 0),
            PieceColor::White,
            PieceType::Bishop,
            &mut game_state,
        );
    }
    for i in &[1, 6] {
        spawn_piece(
            &mut commands,
            game_textures.knightl.clone(),
            Position(*i, 0),
            PieceColor::White,
            PieceType::Knight,
            &mut game_state,
        );
    }
    for i in 0..8 {
        spawn_piece(
            &mut commands,
            game_textures.pawnl.clone(),
            Position(i, 1),
            PieceColor::White,
            PieceType::Pawn,
            &mut game_state,
        );
    }
    spawn_piece(
        &mut commands,
        game_textures.kingd.clone(),
        Position(4, 7),
        PieceColor::Black,
        PieceType::King,
        &mut game_state,
    );
    spawn_piece(
        &mut commands,
        game_textures.queend.clone(),
        Position(3, 7),
        PieceColor::Black,
        PieceType::Queen,
        &mut game_state,
    );
    for i in &[0, 7] {
        spawn_piece(
            &mut commands,
            game_textures.rookd.clone(),
            Position(*i, 7),
            PieceColor::Black,
            PieceType::Rook,
            &mut game_state,
        );
    }
    for i in &[2, 5] {
        spawn_piece(
            &mut commands,
            game_textures.bishopd.clone(),
            Position(*i, 7),
            PieceColor::Black,
            PieceType::Bishop,
            &mut game_state,
        );
    }
    for i in &[1, 6] {
        spawn_piece(
            &mut commands,
            game_textures.knightd.clone(),
            Position(*i, 7),
            PieceColor::Black,
            PieceType::Knight,
            &mut game_state,
        );
    }
    for i in 0..8 {
        spawn_piece(
            &mut commands,
            game_textures.pawnd.clone(),
            Position(i, 6),
            PieceColor::Black,
            PieceType::Pawn,
            &mut game_state,
        );
    }
}
