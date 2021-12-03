use crate::ball::Ball;
use crate::draw::{draw_rectangle, draw_text};
use crate::paddle::{Direction, Paddle};
use piston_window::{types::Color, Context, G2d, Glyphs, Key};

const BORDER_COLOR: Color = [1.0, 1.0, 1.0, 1.0];
const GAMEOVER_COLOR: Color = [0.80, 0.0, 0.0, 0.5];

const MARGIN_TOP: f64 = 5.0;
const MOVING_PERIOD: f64 = 0.08;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    player: Paddle,
    enemy: Paddle,

    ball: Ball,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
    ai_response_time: f64,
    ai_update_time: f64,

    active_key: Option<Key>,
    score: i32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            player: Paddle::new(width as f64 - 3.0, MARGIN_TOP + 5.0, 5),
            enemy: Paddle::new(3.0, MARGIN_TOP + 9.0, 5),
            waiting_time: 0.0,
            ai_response_time: 0.01,
            ai_update_time: 0.0,
            ball: Ball::new(6.0, MARGIN_TOP + 4.0, 100.0, 0.0),
            width,
            height,
            game_over: false,
            active_key: None,
            score: 0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }
        self.active_key = Some(key);
    }

    pub fn key_released(&mut self) {
        self.active_key = None;
    }

    pub fn draw(&self, con: &Context, g: &mut G2d, cache: &mut Glyphs) {
        self.player.draw(con, g);
        self.enemy.draw(con, g);

        // Draw ball
        if !self.game_over {
            self.ball.draw(con, g);
        }

        draw_rectangle(BORDER_COLOR, 0.0, MARGIN_TOP, self.width, 1, con, g);
        draw_rectangle(
            BORDER_COLOR,
            0.0,
            (self.height - 1) as f64,
            self.width,
            1,
            con,
            g,
        );
        draw_rectangle(BORDER_COLOR, 0.0, MARGIN_TOP, 1, self.height, con, g);
        draw_rectangle(
            BORDER_COLOR,
            (self.width - 1) as f64,
            MARGIN_TOP,
            1,
            self.height,
            con,
            g,
        );

        draw_text(
            std::format!("SCORE: {}", self.score).as_str(),
            5.0,
            con,
            g,
            cache,
        );

        if self.game_over {
            draw_rectangle(
                GAMEOVER_COLOR,
                0.0,
                MARGIN_TOP,
                self.width,
                self.height,
                con,
                g,
            );
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_ball(delta_time);
            self.update_player(self.get_dir());
            self.update_ai(delta_time);
            self.waiting_time = 0.0;
        }
    }

    fn update_ball(&mut self, delta_time: f64) {
        let (next_x, next_y) = self.ball.get_next_location(delta_time);

        if next_x > self.width as f64 || next_x < 0.0 {
            // Horizontal wall hit
            // Calculate score
            // Re-Spwan
            self.game_over = true;
            if next_x > self.player.get_position_x() + 1_f64 {
                // GAME OVER
                self.game_over = true;
            } else {
                self.ball.set_velocity(100.0, 0.0);
                self.ball.set_position(6.0, 6.0 + MARGIN_TOP);
                self.score += 1;
            }
        }

        if self.game_over {
            return;
        }

        if next_y > (self.height - 1) as f64 || next_y < MARGIN_TOP + 1.0 {
            // Vertical wall hit
            // change y velocity
            self.ball.flip_velocity_y();
        }

        // Collision Detection
        // Player collision
        if next_x.floor() >= (self.player.get_position_x() - 1.0)
            && next_y >= self.player.get_position_y()
            && next_y <= self.player.get_position_y() + self.player.get_size() as f64
        {
            let paddle_center = self.player.get_position_y() + (self.player.get_size() / 2) as f64;
            let d = paddle_center as f64 - next_y;
            self.ball.flip_velocity_x();
            self.ball.increase_y(d * -20.0);
        }

        // AI collision
        if next_x.ceil() <= (self.enemy.get_position_x() + 1.0)
            && next_y >= self.enemy.get_position_y()
            && next_y <= self.enemy.get_position_y() + self.enemy.get_size() as f64
        {
            let paddle_center = self.enemy.get_position_y() + (self.enemy.get_size() / 2) as f64;
            let d = paddle_center as f64 - next_y;
            self.ball.flip_velocity_x();
            self.ball.increase_y(d * -20.0);
        }

        self.ball.set_position(next_x, next_y);
    }

    fn update_player(&mut self, dir: Option<Direction>) {
        let min_y = MARGIN_TOP;
        let max_y = self.height as f64;
        self.player.slide(dir, min_y, max_y);
    }

    fn get_dir(&self) -> Option<Direction> {
        match self.active_key {
            Some(Key::Up) => Some(Direction::Up),
            Some(Key::Down) => Some(Direction::Down),
            _ => None,
        }
    }

    fn update_ai(&mut self, delta_time: f64) {
        self.ai_update_time += delta_time;
        if self.ai_update_time < self.ai_response_time {
            return;
        }
        self.ai_update_time = 0.0;

        let (_, next_y) = self.ball.get_next_location(delta_time);

        let mut dir: Option<Direction> = None;
        if self.ball.get_velocity_x() < 0.0 {
            if next_y < self.enemy.get_position_y() {
                dir = Some(Direction::Up);
            } else if next_y > self.enemy.get_position_y() + self.enemy.get_size() as f64 {
                dir = Some(Direction::Down);
            }
        }

        let min_y = MARGIN_TOP;
        let max_y = self.height as f64;
        self.enemy.slide(dir, min_y, max_y);
    }

    fn restart(&mut self) {
        self.waiting_time = 0.0;
        self.ball.set_velocity(100.0, 0.0);
        self.ball.set_position(6.0, (self.height / 2) as f64);
        self.game_over = false;
    }
}
