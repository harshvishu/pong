use piston_window::{rectangle, text, types::Color, Context, G2d, Glyphs, Transformed};

const BLOCK_SIZE: f64 = 10.0; // ①
const FONT_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

pub fn to_coord(game_coord: f64) -> f64 {
    // ②
    game_coord * BLOCK_SIZE
}

pub fn draw_block(color: Color, x: f64, y: f64, con: &Context, g: &mut G2d) {
    // ③
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: f64,
    y: f64,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    // ④
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}

pub fn draw_text(text: &str, position: f64, con: &Context, g: &mut G2d, cache: &mut Glyphs) {
    // ⑤
    let transform = con.transform.trans(position * BLOCK_SIZE, 3.0 * BLOCK_SIZE);
    text::Text::new_color(FONT_COLOR, 20)
        .draw(text, cache, &con.draw_state, transform, g)
        .unwrap();
}
