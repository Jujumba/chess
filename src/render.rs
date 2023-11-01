use sdl2::{
    rect::Rect,
    render::Canvas,
    video::Window,
};

use crate::core::{
    piece::{self, Color, Piece, PieceKind}, BoardPosition,
};

pub const WHITE: sdl2::pixels::Color = sdl2::pixels::Color::RGB(237, 208, 140); // old - 242, 222, 179
pub const BLACK: sdl2::pixels::Color = sdl2::pixels::Color::RGB(64, 57, 45);

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub height: i32,
    pub width: i32,
}

impl From<&Window> for Tile {
    fn from(window: &Window) -> Self {
        let mut height = 0;
        let mut width = 0;
        unsafe {
            sdl2_sys::SDL_GetWindowSize(window.raw(), &mut width, &mut height);
        }
        let height = height / 8;
        let width = width / 8;
        Tile { height, width }
    }
}

pub trait Render {
    fn render(&self, canvas: &mut Canvas<Window>, texture: Option<&sdl2::render::Texture>);
}
impl Render for Piece {
    fn render(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture: Option<&sdl2::render::Texture>,
    ) {
        let Tile { height, width } = canvas.window().into();
        let offset: TextureOffset = self.kind.into();
        canvas
            .copy_ex(
                texture.expect("Must provide texture in order to render pieces!"),
                Rect::new(200 * offset.0, 200 * offset.1, 200, 200),
                Rect::new(
                    width * i32::from(self.pos.x),
                    height * i32::from(self.pos.y),
                    width.try_into().unwrap(),
                    height.try_into().unwrap(),
                ),
                0.0,
                None,
                false,
                false,
            )
            .unwrap();
    }
}
impl Render for BoardPosition {
    fn render(&self, canvas: &mut Canvas<Window>, _texture: Option<&sdl2::render::Texture>) {
        let i8intoi32 = <i8 as Into<i32>>::into;
        let tile: Tile = canvas.window().into();
        let p = Rect::new(tile.width * i8intoi32(self.x), tile.height * i8intoi32(self.y), (tile.width / 2) as _, (tile.height / 2) as _);
        canvas.set_draw_color(sdl2::pixels::Color::RED);
        canvas.draw_rect(p).unwrap();
    }
}
pub struct TextureOffset(pub i32, pub i32);
impl From<(i32, i32)> for TextureOffset {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0, value.1)
    }
}

#[allow(clippy::from_over_into)]
impl Into<TextureOffset> for PieceKind {
    fn into(self) -> TextureOffset {
        use piece::PieceKind::*;
        let vertical_offset = if self.color() == Color::Black { 1 } else { 0 };
        match self {
            King(_) => (0, vertical_offset).into(),
            Queen(_) => (1, vertical_offset).into(),
            Bishop(_) => (2, vertical_offset).into(),
            Knight(_) => (3, vertical_offset).into(),
            Rook(_) => (4, vertical_offset).into(),
            Pawn(_) => (5, vertical_offset).into(),
        }
    }
}
