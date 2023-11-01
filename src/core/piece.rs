use std::ops::{Deref, DerefMut};


use super::BoardPosition;

#[derive(Debug, Clone, Copy)]
struct PieceId(i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    Pawn(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Queen(Color),
    King(Color),
}
impl PieceKind {
    pub fn color(self) -> Color {
        use PieceKind::*;
        match self {
            Pawn(color) | Rook(color) | Knight(color) | Bishop(color) | Queen(color) | King(color) => color
        }
    }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Piece {
    pub pos: BoardPosition,
    pub kind: PieceKind,
}
impl Piece {
    pub fn new(x: i8, y: i8, kind: PieceKind) -> Self {
        Self { pos:  (x, y).try_into().expect("Piece is out of board!"), kind } // todo: add check!
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pieces(Vec<Piece>);

impl Deref for Pieces {
    type Target = Vec<Piece>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Pieces {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
// impl IntoIterator for Pieces {
//     type IntoIter = <Vec<Piece> as IntoIterator>::IntoIter;
//     type Item = <Vec<Piece> as IntoIterator>::Item;
//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }
// impl Pieces {
//     pub fn iter(&self) -> std::slice::Iter<'_, Piece> {
//         self.0.iter()
//     }
//     pub fn remove_piece_by_pos(&mut self, pos: BoardPosition) -> Result<(), ChessError> {
//         let i = self.iter().position(|p: &Piece| p.pos == pos).ok_or(ChessError::NoSuchPiece)?;
//         let _ = self.0.remove(i);
//         Ok(())
//     }
//     pub fn remove_piece(&mut self, piece: Piece) -> Result<(), ChessError> {
//         self.remove_piece_by_pos(piece.pos)
//     }
//     pub fn move_piece(&mut self, mut piece: Piece, pos: BoardPosition) -> Piece {
//         self.remove_piece(piece).unwrap();
//         piece.pos = pos;
//         self.0.push(piece);
//         piece
//         // todo
//     }
// }
impl Default for Pieces {
    fn default() -> Self {
        let mut pieces = Vec::with_capacity(32);
        for color in [Color::Black, Color::White] {
            let pawn_y = if color == Color::Black { 1 } else { 6 };
            for pawn_x in 0..8 {
                pieces.push(Piece::new(pawn_x, pawn_y, PieceKind::Pawn(color)));
            }
            let piece_y = if color == Color::Black { 0 } else { 7 };
            pieces.push(Piece::new(0, piece_y, PieceKind::Rook(color)));
            pieces.push(Piece::new(7, piece_y, PieceKind::Rook(color)));
            pieces.push(Piece::new(1, piece_y, PieceKind::Knight(color)));
            pieces.push(Piece::new(6, piece_y, PieceKind::Knight(color)));
            pieces.push(Piece::new(2, piece_y, PieceKind::Bishop(color)));
            pieces.push(Piece::new(5, piece_y, PieceKind::Bishop(color)));
            pieces.push(Piece::new(3, piece_y, PieceKind::King(color)));
            pieces.push(Piece::new(4, piece_y, PieceKind::Queen(color)));
        }
        Self(pieces)
    }
}