use bevy::ecs::component::Component;

use crate::piece_square_tables::*;

use crate::common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct Piece {
    pub piece_color: PieceColor,
    pub piece_type: PieceType,
    pub x: i8,
    pub y: i8,
}

pub type Board = [[Option<Piece>; 8]; 8];

impl Piece {
    fn table_position(&self) -> Position {
        match self.piece_color {
            PieceColor::White => Position(7 - self.y, self.x),
            PieceColor::Black => Position(self.y, self.x),
        }
    }
    pub fn value(&self, is_endgame: bool) -> f32 {
        let Position(x, y) = self.table_position();
        let (x, y) = (x as usize, y as usize);
        let value = match self.piece_type {
            PieceType::King => {
                INFINITY
                    + if is_endgame {
                        TABLE_KING_END_GAME[x][y] as f32
                    } else {
                        TABLE_KING_MIDDLE_GAME[x][y] as f32
                    }
            }
            PieceType::Queen => 900. + TABLE_QUEEN[x][y] as f32,
            PieceType::Rook => 500. + TABLE_ROOK[x][y] as f32,
            PieceType::Bishop => 330. + TABLE_BISHOP[x][y] as f32,
            PieceType::Knight => 320. + TABLE_KNIGHT[x][y] as f32,
            PieceType::Pawn => 100. + TABLE_PAWN[x][y] as f32,
        };
        match self.piece_color {
            PieceColor::White => value,
            PieceColor::Black => -value,
        }
    }
    fn is_legal(&self, x: i8, y: i8, board: Board) -> bool {
        if !(0..8).contains(&x) || !(0..8).contains(&y) {
            return false;
        }
        if let Some(piece) = board[x as usize][y as usize] {
            self.piece_color != piece.piece_color
        } else {
            true
        }
    }
    fn is_capture(&self, x: i8, y: i8, board: Board) -> bool {
        board[x as usize][y as usize].is_some()
    }
    fn gen_consecutive(&self, board: Board, diffs: Vec<(i8, i8)>) -> Vec<Move> {
        let mut legal_moves = Vec::new();
        for (dx, dy) in diffs {
            let mut x = self.x + dx;
            let mut y = self.y + dy;
            while self.is_legal(x, y, board) {
                legal_moves.push((Position(self.x, self.y), Position(x, y)));
                if self.is_capture(x, y, board) {
                    break;
                }
                x += dx;
                y += dy;
            }
        }
        legal_moves
    }
    fn gen_rook(&self, board: Board) -> Vec<Move> {
        self.gen_consecutive(board, vec![(1, 0), (-1, 0), (0, 1), (0, -1)])
    }
    fn gen_bishop(&self, board: Board) -> Vec<Move> {
        self.gen_consecutive(board, vec![(1, 1), (1, -1), (-1, 1), (-1, -1)])
    }
    pub fn gen_legal_moves(&self, board: Board) -> Vec<Move> {
        let mut legal_moves: Vec<Move> = Vec::new();

        match self.piece_type {
            PieceType::King => {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if self.is_legal(self.x + dx, self.y + dy, board) {
                            legal_moves.push((
                                Position(self.x, self.y),
                                Position(self.x + dx, self.y + dy),
                            ));
                        }
                    }
                }
                let row: usize = match self.piece_color {
                    PieceColor::White => 0,
                    PieceColor::Black => 7,
                };
                if self.x == 4 && self.y == row as i8 {
                    if let Some(piece) = board[0][row] {
                        if piece.piece_type == PieceType::Rook
                            && piece.piece_color == self.piece_color
                            && board[1][row].is_none()
                            && board[2][row].is_none()
                            && board[3][row].is_none()
                        {
                            legal_moves
                                .push((Position(self.x, self.y), Position(self.x - 2, self.y)));
                        }
                    }
                    if let Some(piece) = board[7][row] {
                        if piece.piece_type == PieceType::Rook
                            && piece.piece_color == self.piece_color
                            && board[6][row].is_none()
                            && board[5][row].is_none()
                        {
                            legal_moves
                                .push((Position(self.x, self.y), Position(self.x + 2, self.y)));
                        }
                    }
                }
            }
            PieceType::Queen => {
                legal_moves.append(&mut self.gen_rook(board));
                legal_moves.append(&mut self.gen_bishop(board));
            }
            PieceType::Rook => {
                legal_moves.append(&mut self.gen_rook(board));
            }
            PieceType::Bishop => {
                legal_moves.append(&mut self.gen_bishop(board));
            }
            PieceType::Knight => {
                for dx in -2..=2 {
                    for dy in -2..=2 {
                        if i8::abs(dx * dy) == 2 && self.is_legal(self.x + dx, self.y + dy, board) {
                            legal_moves.push((
                                Position(self.x, self.y),
                                Position(self.x + dx, self.y + dy),
                            ));
                        }
                    }
                }
            }
            PieceType::Pawn => {
                let dir = match self.piece_color {
                    PieceColor::White => 1,
                    PieceColor::Black => -1,
                };
                for dx in &[-1, 1] {
                    if self.is_legal(self.x + dx, self.y + dir, board)
                        && self.is_capture(self.x + dx, self.y + dir, board)
                    {
                        legal_moves.push((
                            Position(self.x, self.y),
                            Position(self.x + dx, self.y + dir),
                        ));
                    }
                }
                if self.is_legal(self.x, self.y + dir, board)
                    && !self.is_capture(self.x, self.y + dir, board)
                {
                    legal_moves.push((Position(self.x, self.y), Position(self.x, self.y + dir)));
                    if self.y == (-25 * dir + 35) / 10
                        && self.is_legal(self.x, self.y + 2 * dir, board)
                        && !self.is_capture(self.x, self.y + 2 * dir, board)
                    {
                        legal_moves
                            .push((Position(self.x, self.y), Position(self.x, self.y + 2 * dir)));
                    }
                }
            }
        }
        legal_moves
    }
    pub fn move_piece(&mut self, x: i8, y: i8) {
        self.x = x;
        self.y = y;
    }
}
