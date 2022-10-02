use crate::common::*;
use crate::game_textures::*;
use crate::physical_board::*;
use crate::piece::*;
use crate::program_options::*;
use crate::spawn_tile::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GameState {
    pub board: Board,
    pub now_moves: PieceColor,
    pub player_moves: bool,
}

impl GameState {
    pub fn stats(&self) -> (bool, bool, bool) {
        let mut white_king = false;
        let mut black_king = false;
        let mut white_queen = 0;
        let mut black_queen = 0;
        let mut white_other_pieces = 0;
        let mut black_other_pieces = 0;
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.board[i][j] {
                    match (piece.piece_type, piece.piece_color) {
                        (PieceType::King, PieceColor::White) => white_king = true,
                        (PieceType::King, PieceColor::Black) => black_king = true,
                        (PieceType::Queen, PieceColor::White) => white_queen += 1,
                        (PieceType::Queen, PieceColor::Black) => black_queen += 1,
                        (PieceType::Pawn, _) => {}
                        (_, PieceColor::White) => white_other_pieces += 1,
                        (_, PieceColor::Black) => black_other_pieces += 1,
                    }
                }
            }
        }
        let is_endgame = (white_queen == 0 || white_other_pieces <= 1)
            && (black_queen == 0 || black_other_pieces <= 1);
        (white_king, black_king, is_endgame)
    }
    fn cache_insert(
        &self,
        level: i32,
        cache: &mut HashMap<(GameState, i32), f32>,
        key: (GameState, i32),
        value: f32,
    ) {
        unsafe {
            if level == DEPTH - 1 || level <= DEPTH - 4 {
                cache.insert(key, value);
            }
        }
    }
    fn evaluate_static(&self, is_endgame: bool) -> f32 {
        let mut score = 0.;
        for i in 0..8 {
            for j in 0..8 {
                if let Some(x) = self.board[i][j] {
                    score += x.value(is_endgame);
                }
            }
        }
        score
    }
    pub fn evaluate(
        &self,
        level: i32,
        cache: &mut HashMap<(GameState, i32), f32>,
        mut alpha: f32,
        mut beta: f32,
    ) -> f32 {
        let key = (self.clone(), level);
        if let Some(x) = cache.get(&key) {
            *x
        } else {
            let (white_king, black_king, is_endgame) = self.stats();
            if !white_king {
                let value = -2. * INFINITY - level as f32 / 10.;
                self.cache_insert(level, cache, key, value);
                return value;
            }
            if !black_king {
                let value = 2. * INFINITY + level as f32 / 10.;
                self.cache_insert(level, cache, key, value);
                return value;
            }
            let value = if level > 0 {
                let mut score = match self.now_moves {
                    PieceColor::White => -BIG_INFINITY,
                    PieceColor::Black => BIG_INFINITY,
                };
                for (from, to) in self.generate_legal_moves() {
                    let mut next_state = self.clone();
                    next_state.move_piece(from, to, true);
                    let next_state_score = next_state.evaluate(level - 1, cache, alpha, beta);
                    match self.now_moves {
                        PieceColor::White => {
                            score = score.max(next_state_score);
                            if score > beta {
                                break;
                            }
                            alpha = alpha.max(score);
                        }
                        PieceColor::Black => {
                            score = score.min(next_state_score);
                            if score < alpha {
                                break;
                            }
                            beta = beta.min(score);
                        }
                    };
                }
                score
            } else {
                self.evaluate_static(is_endgame)
            };
            self.cache_insert(level, cache, key, value);
            value
        }
    }
    fn is_capture(&self, _from: Position, to: Position) -> bool {
        self.board[to.0 as usize][to.1 as usize].is_some()
    }
    pub fn generate_legal_moves(&self) -> Vec<Move> {
        let mut legal_moves = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.board[i][j] {
                    if piece.piece_color == self.now_moves {
                        legal_moves.append(&mut piece.generate_legal_moves(self.board));
                    }
                }
            }
        }
        legal_moves.sort_unstable_by(|(afrom, ato), (bfrom, bto)| {
            self.is_capture(*bfrom, *bto)
                .cmp(&self.is_capture(*afrom, *ato))
        });
        legal_moves
    }
    pub fn move_piece(&mut self, from: Position, to: Position, change_now_moves: bool) {
        let mut piece = self.board[from.0 as usize][from.1 as usize].take().unwrap();
        if piece.piece_type == PieceType::King {
            if from.0 + 2 == to.0 {
                self.move_piece(Position(7, from.1), Position(5, to.1), false);
            }
            if from.0 - 2 == to.0 {
                self.move_piece(Position(0, from.1), Position(3, to.1), false);
            }
        }
        if piece.piece_type == PieceType::Pawn && (to.1 == 0 || to.1 == 7) {
            piece.piece_type = PieceType::Queen;
        }
        piece.move_piece(to.0, to.1);
        self.board[to.0 as usize][to.1 as usize] = Some(piece);
        if change_now_moves {
            self.now_moves = match self.now_moves {
                PieceColor::White => PieceColor::Black,
                PieceColor::Black => PieceColor::White,
            };
        }
    }
    fn move_piece_for_real(
        &mut self,
        mut game_textures: Res<GameTextures>,
        commands: &mut Commands,
        mut query: Query<(Entity, &mut Position, &mut Transform, &mut Handle<Image>)>,
        from: Position,
        to: Position,
    ) {
        spawn_tile(commands, game_textures.highlight.clone(), from, true);
        spawn_tile(commands, game_textures.highlight.clone(), to, true);
        let piece_type = self.board[from.0 as usize][from.1 as usize]
            .unwrap()
            .piece_type;
        if piece_type == PieceType::Pawn && (to.1 == 0 || to.1 == 7) {
            move_piece_physically(&mut game_textures, commands, &mut query, from, to, true);
        } else {
            move_piece_physically(&mut game_textures, commands, &mut query, from, to, false);
        }
        if piece_type == PieceType::King {
            if from.0 + 2 == to.0 {
                move_piece_physically(
                    &mut game_textures,
                    commands,
                    &mut query,
                    Position(7, from.1),
                    Position(5, to.1),
                    false,
                );
            }
            if from.0 - 2 == to.0 {
                move_piece_physically(
                    &mut game_textures,
                    commands,
                    &mut query,
                    Position(0, from.1),
                    Position(3, to.1),
                    false,
                );
            }
        }
        self.move_piece(from, to, true);
        unsafe {
            if NUMBER_OF_PLAYERS == 1 {
                self.player_moves = !self.player_moves;
            }
        }
    }
    pub fn player_move(
        &mut self,
        game_textures: Res<GameTextures>,
        commands: &mut Commands,
        query: Query<(Entity, &mut Position, &mut Transform, &mut Handle<Image>)>,
        from: Position,
        to: Position,
    ) {
        if !self.player_moves || !self.generate_legal_moves().contains(&(from, to)) {
            return;
        }
        self.move_piece_for_real(game_textures, commands, query, from, to);
    }
    pub fn computer_move(
        &mut self,
        game_textures: Res<GameTextures>,
        commands: &mut Commands,
        query: Query<(Entity, &mut Position, &mut Transform, &mut Handle<Image>)>,
        from: Position,
        to: Position,
    ) {
        assert!(!self.player_moves && self.generate_legal_moves().contains(&(from, to)));
        self.move_piece_for_real(game_textures, commands, query, from, to);
    }
}
