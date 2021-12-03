use crate::draw::draw_block;
use piston_window::{types::Color, Context, G2d};

const BALL_COLOR: Color = [1.0, 1.0, 1.0, 1.0];

pub struct Ball {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

impl Ball {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64) -> Self {
        Self { x, y, vx, vy }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_block(BALL_COLOR, self.x, self.y, con, g);
    }

    pub fn set_position(&mut self, x: f64, y: f64) {    // ①
        self.x = x;
        self.y = y;
    }

    pub fn get_next_location(&self, delta_time: f64) -> (f64, f64) {    // ②
        let distance_x = self.vx * delta_time;
        let distance_y = self.vy * delta_time;
        let new_x = self.x + distance_x;
        let new_y = self.y + distance_y;
        (new_x, new_y)
    }

    pub fn flip_velocity_y(&mut self) {  // ③
        self.vy *= -1.0;
    }

    pub fn flip_velocity_x(&mut self) {  
        self.vx *= -1.0;
    }

    pub fn increase_y(&mut self, factor: f64) {     // ④ 
        self.vy += factor;
    }

    pub fn get_velocity_x(&self) -> f64 {   // ⑤
        self.vx
    }

    pub fn set_velocity(&mut self, vx: f64, vy: f64) {  // ⑥
        self.vx = vx;
        self.vy = vy;
    }
}
