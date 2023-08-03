use ggez::conf::WindowMode;
use ggez::event::EventHandler;
use ggez::graphics::{
    Canvas, Color, DrawMode, DrawParam, FillOptions, Mesh, Rect, Text, TextLayout,
};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};
mod ball;
mod players;
mod side;
use crate::game::ball::Ball;
use crate::game::players::Player;
use crate::game::side::Side;

struct Score {
    p1: i32,
    p2: i32,
}

impl Score {
    pub fn new() -> Self {
        Self { p1: 0, p2: 0 }
    }

    pub fn add_score(&mut self, side: Side) {
        match side {
            Side::Left => self.p1 += 1,
            Side::Right => self.p2 += 1,
            _ => (),
        }
    }
}

pub struct MyGame {
    player1: Player,
    player2: Player,
    ball: Ball,
    score: Score,
    expected_collision: Side,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> Self {
        // Load/create resources such as images here.
        let window_mode = WindowMode::default();
        let window_size = (window_mode.width, window_mode.height);

        MyGame {
            player1: Player::new(Side::Left, window_size, true).unwrap(),
            player2: Player::new(Side::Right, window_size, false).unwrap(),
            ball: Ball::new(window_size),
            score: Score::new(),
            expected_collision: Side::Up,
        }
    }

    pub fn next_round(&mut self, window_size: (f32, f32)) {
        self.player1 = Player::new(Side::Left, window_size, true).unwrap();
        self.player2 = Player::new(Side::Right, window_size, false).unwrap();
        self.ball = Ball::new(window_size);
        self.expected_collision = Side::Up;
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let speed_players = 4.;

        let (window_x, window_y) = (WindowMode::default().width, WindowMode::default().height);
        let window_size = (window_x, window_y);

        let move_keys = Vec::from([KeyCode::W, KeyCode::S, KeyCode::Up, KeyCode::Down]);
        for i in 0..move_keys.len() {
            if _ctx.keyboard.is_key_pressed(move_keys[i]) && !self.player1.bot {
                match i {
                    0 => self.player1.y_move(Side::Up, speed_players, window_y),
                    1 => self.player1.y_move(Side::Down, speed_players, window_y),
                    _ => (),
                }
            }
            if _ctx.keyboard.is_key_pressed(move_keys[i]) && !self.player2.bot {
                match i {
                    2 => self.player2.y_move(Side::Up, speed_players, window_y),
                    3 => self.player2.y_move(Side::Down, speed_players, window_y),
                    _ => (),
                }
            }
        }

        let result = self.ball.update(window_size);
        match result {
            Ok(Side::Left) | Ok(Side::Right) => {
                self.score.add_score(result.unwrap());
                self.next_round(window_size);
            }
            _ => (),
        }

        match self.expected_collision {
            Side::Left => {
                if self.player1.collision(&mut self.ball) == Ok(()) {
                    self.expected_collision = Side::Right;
                }
            },
            Side::Right => {
                if self.player2.collision(&mut self.ball) == Ok(()) {
                    self.expected_collision = Side::Left;
                }
            },
            _ => {
                if self.player1.collision(&mut self.ball) == Ok(()) {
                    self.expected_collision = Side::Right;
                }
                if self.player2.collision(&mut self.ball) == Ok(()) {
                    self.expected_collision = Side::Left;
                }
            }
        }

        self.player1.bot_move(&self.ball, window_y);
        self.player2.bot_move(&self.ball, window_y);
        self.ball.x_speed *= 1.001;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        let (window_x, window_y) = (WindowMode::default().width, WindowMode::default().height);
        let player1 = Mesh::new_rectangle(
            ctx,
            DrawMode::Fill(FillOptions::DEFAULT),
            Rect {
                x: self.player1.x1,
                y: self.player1.y1,
                w: self.player1.width,
                h: self.player1.height,
            },
            Color::WHITE,
        )
        .unwrap();
        let player2 = Mesh::new_rectangle(
            ctx,
            DrawMode::Fill(FillOptions::DEFAULT),
            Rect {
                x: self.player2.x1,
                y: self.player2.y1,
                w: self.player2.width,
                h: self.player2.height,
            },
            Color::WHITE,
        )
        .unwrap();
        let ball = Mesh::new_rectangle(
            ctx,
            DrawMode::Fill(FillOptions::DEFAULT),
            Rect {
                x: self.ball.x,
                y: self.ball.y,
                w: self.ball.r,
                h: self.ball.r,
            },
            Color::WHITE,
        )
        .unwrap();

        let mut scores = Text::new(format!("{}  {}", self.score.p1, self.score.p2));
        scores.set_layout(TextLayout::center());
        canvas.draw(&player1, DrawParam::new());
        canvas.draw(&player2, DrawParam::new());
        canvas.draw(&ball, DrawParam::new());
        //scores.set_font(Font::default(), Scale::uniform(24.0));
        canvas.draw(
            &scores,
            DrawParam::default().dest([
                window_x / 2.0 - scores.glyph_positions(ctx).unwrap()[0].x as f32 / 2.0,
                window_y / 10.0,
            ]),
        );
        canvas.finish(ctx)
    }
}
