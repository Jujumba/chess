pub mod piece;

use sdl2::{
    rect::Rect,
    render::{Canvas, Texture},
    video::Window,
};

use crate::render::{Render, Tile, WHITE, BLACK};

use self::piece::{Color, Piece, PieceKind, Pieces};

macro_rules! iterate_on_rank_or_file {
    ($board: expr, $legal_moves: expr, $sub_coord: expr, $add_coord: expr, $piece: expr, $coordinate: tt $(,)?) => {
        let mut coord = 1;
        while coord < 8 {
            let (function, coordinate): (fn(_, _) -> _, i8) = match coord <= $piece.pos.$coordinate {
                true => ($sub_coord, coord),
                false => ($add_coord, coord - $piece.pos.$coordinate),
            };
            if let Some(pos) = function($piece.pos, coordinate) {
                match $board.find(pos) {
                    None => $legal_moves.push(pos),
                    Some(other) => {
                        let a = other.kind.color() != $piece.kind.color();
                        if a {
                            $legal_moves.push(pos);
                        }
                        if coord > $piece.pos.$coordinate {
                            break;
                        } else {
                            coord = $piece.pos.$coordinate + 1;
                            continue;
                        }
                    },
                }
            }
            coord += 1;
        }
    };
}

macro_rules! iterate_over_diagonal {
    ($board: expr, $legal_moves: expr, $piece: expr, $x_step: expr, $y_step: expr) => {
        let mut initial = $piece.pos;
        while BoardPosition::in_board(initial.x + $x_step) && BoardPosition::in_board(initial.y + $y_step) {
            initial = initial.try_add_x($x_step).unwrap().try_add_y($y_step).unwrap();
            match $board.find(initial) {
                None => $legal_moves.push(initial),
                Some(other) if other.kind.color() != $piece.kind.color() => {
                    $legal_moves.push(initial);
                    break;
                }
                _ => break,
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct Board {
    pub(self) pieces: Pieces,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoardPosition {
    // Vary from 0 to 7
    pub x: i8,
    pub y: i8,
}
impl Board {
    pub fn find(&self, pos: BoardPosition) -> Option<Piece> {
        let BoardPosition { x, y } = pos;
        self.pieces
            .iter()
            .find(|piece| piece.pos.x == x && piece.pos.y == y)
            .map(Clone::clone)
    }
    pub fn opponent_on(&self, pos: BoardPosition, piece: Piece) -> Option<Piece> {
        match self.find(pos) {
            Some(other) if other.kind.color() == piece.kind.color() => Some(other),
            _ => None
        }
    }
    pub fn remove_peice_on(&mut self, pos: BoardPosition) -> Result<(), ChessError> {
        let index = self.pieces.iter().position(|piece| piece.pos == pos).ok_or(ChessError::NoSuchPiece)?;
        self.pieces.remove(index);
        Ok(())
    }
    pub fn get_legal_moves(&self, piece: Piece) -> Vec<BoardPosition> {
        let mut legal_moves = Vec::with_capacity(8);
        match piece.kind {
            PieceKind::Pawn(color) => {
                let max_y: i8 = if piece.pos.y == 1 || piece.pos.y == 6 { 2 } else { 1 };
                for y in 1..=max_y {
                    let function = match color { 
                        Color::Black => BoardPosition::try_add_y,
                        Color::White => BoardPosition::try_sub_y,
                    };
                    if let Some(pos) = function(piece.pos, y) {
                        if self.find(pos).is_none() {
                            legal_moves.push(pos);
                        }
                    }
                }
            }
            PieceKind::Rook(_) => {
                iterate_on_rank_or_file!(self, legal_moves, BoardPosition::try_sub_x, BoardPosition::try_add_x, piece, x);
                iterate_on_rank_or_file!(self, legal_moves, BoardPosition::try_sub_y, BoardPosition::try_add_y, piece, y);
            }
            PieceKind::Knight(color) => {
                for (x, y) in [(2, 1), (2, -1), (1, 2), (1, -2), (-2, 1), (-2, -1), (-1, 2), (-1, -2)] {
                    if let Some(pos) = piece.pos.try_add(x, y) {
                        match self.find(pos) {
                            Some(piece) if piece.kind.color() != color => legal_moves.push(pos),
                            None => legal_moves.push(pos),
                            Some(_) => (),
                        }
                    }
                }
            }
            PieceKind::Bishop(_) => {
                iterate_over_diagonal!(self, legal_moves, piece, 1, 1);
                iterate_over_diagonal!(self, legal_moves, piece, -1, 1);
                iterate_over_diagonal!(self, legal_moves, piece, 1, -1);
                iterate_over_diagonal!(self, legal_moves, piece, -1, -1);
            }
            PieceKind::Queen(_) => {
                iterate_on_rank_or_file!(self, legal_moves, BoardPosition::try_sub_x, BoardPosition::try_add_x, piece, x);
                iterate_on_rank_or_file!(self, legal_moves, BoardPosition::try_sub_y, BoardPosition::try_add_y, piece, y);
                iterate_over_diagonal!(self, legal_moves, piece, 1, 1);
                iterate_over_diagonal!(self, legal_moves, piece, -1, 1);
                iterate_over_diagonal!(self, legal_moves, piece, 1, -1);
                iterate_over_diagonal!(self, legal_moves, piece, -1, -1);
            }
            PieceKind::King(color) => { // todo: disallow to move if it will lead to mate
                for (x, y) in [(1,1), (-1, -1), (1, 0), (-1, 0), (0, 1), (0, -1), (-1, 1), (1, -1)] {
                    let Some(pos) = piece.pos.try_add(x, y) else { 
                        continue
                    };
                    match self.find(pos) {
                        Some(piece) if piece.kind.color() != color => legal_moves.push(pos),
                        None => legal_moves.push(pos),
                        Some(_) => ()
                    }
                }
            }
        }
        legal_moves
    }
}
impl Render for Board {
    fn render(&self, canvas: &mut Canvas<Window>, texture: Option<&Texture>) {
        let Tile { height, width } = canvas.window().into();
        for row in 0..8i32 {
            for column in 0..8i32 {
                let tile = Rect::new(
                    column * width,
                    row * height,
                    width.try_into().unwrap(),
                    height.try_into().unwrap(),
                );
                canvas.set_draw_color(if (row + column) & 1 == 0 {
                    WHITE
                } else {
                    BLACK
                });
                canvas.fill_rect(tile).unwrap();
            }
        }
        for piece in self.pieces.iter() {
            piece.render(canvas, texture);
        }
    }
}
impl BoardPosition {
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }
    pub fn in_board(coordinate: i8) -> bool {
        (0..=7).contains(&coordinate)
    }
    pub fn try_add_x(self, x: i8) -> Option<Self> {
        match Self::in_board(self.x + x) {
            true => Some(
                Self {
                    x: self.x + x,
                    y: self.y,
                }
            ),
            false => None
        }
    }
    pub fn try_sub_x(self, x: i8) -> Option<Self> {
        match Self::in_board(self.x - x) {
            true => Some(
                Self {
                    x: self.x - x,
                    y: self.y,
                }
            ),
            false => None
        }
    }
    pub fn try_add_y(self, y: i8) -> Option<Self> {
        match Self::in_board(self.y + y) {
            true => Some(
                Self {
                    x: self.x,
                    y: self.y + y,
                }
            ),
            false => None
        }
    }
    pub fn try_sub_y(self, y: i8) -> Option<Self> {
        match Self::in_board(self.y - y) {
            true => Some(
                Self {
                    x: self.x,
                    y: self.y - y,
                }
            ),
            false => None
        }
    }
    pub fn try_add(self, x: i8, y: i8) -> Option<Self> {
        match Self::in_board(self.x + x) && Self::in_board(self.y + y) {
            true => Some(
                Self {
                    x: self.x + x,
                    y: self.y + y,
                }
            ),
            false => None
        }
    }
}
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum ChessError {
    OutOfBoard { x: i8, y: i8 },
    NoSuchPiece,
}
impl std::fmt::Display for ChessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self:?}")
    }
}
impl std::error::Error for ChessError { }
impl TryFrom<(i8, i8)> for BoardPosition {
    type Error = ChessError;
    fn try_from(value: (i8, i8)) -> Result<Self, Self::Error> {
        match BoardPosition::in_board(value.0) && BoardPosition::in_board(value.1) {
            true => Ok(Self { x: value.0, y: value.1, }),
            false => Err(ChessError::OutOfBoard { x: value.0, y: value.1, })
        }
    }
}
