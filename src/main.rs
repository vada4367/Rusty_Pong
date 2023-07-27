use ggez::conf::WindowMode;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, FillOptions, Mesh, Rect};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;

enum Side {
    Left,
    Right,
    Up,
    Down,
}

struct Player {
    x1: f32,
    y1: f32,
    width: f32,
    height: f32,
}

struct Ball {
    x: f32,
    y: f32,
    r: f32,
    x_speed: f32,
    y_speed: f32,
}

impl Ball {
    pub fn new(window_size: (f32, f32)) -> Self {
        let radius: f32 = 20.0;
        let mut rng = rand::thread_rng();
        let (x_speed, y_speed) = (
            (3 * (rng.gen::<bool>() as i32 * 2 - 1)) as f32,
            (rng.gen::<f32>() - 0.5) * 10.0,
        );
        let (x, y) = (
            (window_size.0 - radius) / 2.0,
            (window_size.1 - radius) / 2.0,
        );

        Ball {
            x: x,
            y: y,
            r: radius,
            x_speed: x_speed,
            y_speed: y_speed,
        }
    }

    fn screen_collision(&mut self, window_size: (f32, f32)) -> Result<Side, ()> {
        if self.y < 0.0 || self.y + self.r > window_size.1 {
            self.y_speed *= -1.0;
        }
        if self.x + self.r < 0.0 {
            return Ok(Side::Right);
        }
        if self.x > window_size.0 {
            return Ok(Side::Left);
        }

        Err(())
    }

    pub fn update(&mut self, window_size: (f32, f32)) -> Result<Side, ()> {
        let sc_result = self.screen_collision(window_size);
        self.x += self.x_speed;
        self.y += self.y_speed;

        sc_result
    }
}

impl Player {
    pub fn new(side: Side, size_window: (f32, f32)) -> Result<Self, ()> {
        let w: f32 = 20.0;
        let h: f32 = 100.0;
        let x: f32;
        let y: f32 = size_window.1 / 2. - h / 2.;

        match side {
            Side::Left => {
                x = 0.0;
            }

            Side::Right => {
                x = size_window.0 - w;
            }

            _ => {
                return Err(());
            }
        }

        Ok(Player {
            x1: x,
            y1: y,
            height: h,
            width: w,
        })
    }

    fn move_up(&mut self, speed: f32) {
        if self.y1 > speed {
            self.y1 -= speed;
        } else {
            self.y1 = 0.;
        }
    }

    fn move_down(&mut self, speed: f32, window_y: f32) {
        if self.y1 + self.height + speed > window_y {
            self.y1 = window_y - self.height;
        } else {
            self.y1 += speed;
        }
    }

    pub fn y_move(&mut self, side: Side, speed: f32, window_y: f32) {
        match side {
            Side::Up => self.move_up(speed),
            Side::Down => self.move_down(speed, window_y),
            _ => todo!(),
        }
    }
}

struct MyGame {
    player1: Player,
    player2: Player,
    ball: Ball,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> Self {
        // Load/create resources such as images here.
        let window_mode = WindowMode::default();
        let window_size = (window_mode.width, window_mode.height);

        MyGame {
            player1: Player::new(Side::Left, window_size).unwrap(),
            player2: Player::new(Side::Right, window_size).unwrap(),
            ball: Ball::new(window_size),
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let speed_players = 4.;

        let (window_x, window_y) = (WindowMode::default().width, WindowMode::default().height);
        let window_size = (window_x, window_y);

        let move_keys = Vec::from([KeyCode::W, KeyCode::S, KeyCode::Up, KeyCode::Down]);
        for i in 0..move_keys.len() {
            if _ctx.keyboard.is_key_pressed(move_keys[i]) {
                match i {
                    0 => self.player1.y_move(Side::Up, speed_players, window_y),
                    1 => self.player1.y_move(Side::Down, speed_players, window_y),
                    2 => self.player2.y_move(Side::Up, speed_players, window_y),
                    3 => self.player2.y_move(Side::Down, speed_players, window_y),
                    _ => todo!(),
                }
            }
        }

        let result = self.ball.update(window_size);
        match result {
            Ok(Side::Left) => self.ball = Ball::new(window_size),
            Ok(Side::Right) => self.ball = Ball::new(window_size),
            _ => (),
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

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

        canvas.draw(&player1, DrawParam::new());
        canvas.draw(&player2, DrawParam::new());
        canvas.draw(&ball, DrawParam::new());

        canvas.finish(ctx)
    }
}

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Pong", "By God")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}
