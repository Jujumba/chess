use chess::{
    core::{
        Board, BoardPosition, ChessError,
    },
    render::{Render, Tile},
};
use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    mouse::MouseButton,
};
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("Chess", 640, 640)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/PiecesSprites.png")?; // 200x200 for each piece

    #[allow(unused_mut)]
    let mut board = Board::default();
    'outer: loop {
        board.render(&mut canvas, Some(&texture));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'outer,
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    let Ok(_pos) = determine_position(x, y, canvas.window().into()) else {
                        continue
                    };
                }
                _ => (),
            }
        }
        canvas.present();
    }
    Ok(())
}
fn determine_position(x: i32, y: i32, tile: Tile) -> Result<BoardPosition, ChessError> {
    (
        (x / tile.width).try_into().unwrap(),
        (y / tile.height).try_into().unwrap(),
    )
        .try_into()
}