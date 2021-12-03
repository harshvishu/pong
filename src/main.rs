extern crate piston_window;

mod ball;
mod draw;
mod game;
mod paddle;

use piston_window::{
    clear, types::Color, Button, PistonWindow, PressEvent, ReleaseEvent, Size, UpdateEvent,
    WindowSettings,
};

use draw::to_coord;
use game::Game;

const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let (width, height) = (50, 55);

    let mut window: PistonWindow = WindowSettings::new(
        "Ping Pong",
        Size {
            width: to_coord(width as f64),
            height: to_coord(height as f64),
        },
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();

    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        if let Some(Button::Keyboard(_)) = event.release_args() {
            game.key_released();
        }

        window.draw_2d(&event, |c, g, device| {
            clear(BACK_COLOR, g);

            game.draw(&c, g, &mut glyphs);

            glyphs.factory.encoder.flush(device);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
