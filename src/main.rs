use ggez::conf::WindowMode;
use ggez::event;
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, FillOptions, Mesh, Rect, Text, TextLayout};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;

#[derive(PartialEq)]
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
    side: Side,
    bot: bool,
}

struct Ball {
    x: f32,
    y: f32,
    r: f32,
    x_speed: f32,
    y_speed: f32,
}

struct Score {
    p1: i32,
    p2: i32,
}

impl Player {
    pub fn new(side: Side, size_window: (f32, f32), _bot: bool) -> Result<Self, ()> {
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
            side: side,
            bot: _bot,
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
            _ => (),
        }
    }

    fn direct_collision(&self, ball: &mut Ball) {
        if ((self.side == Side::Left && self.x1 + self.width > ball.x) || 
           (self.side == Side::Right && self.x1 < ball.x + ball.r)) && 
            (self.y1 < ball.y && self.y1 + self.height > ball.y + ball.r) 
                { ball.x_speed *= -1.0; }
    }

    fn corner_collision(&self, ball: &mut Ball) -> Result<(), ()> {
        let down_side = self.y1 + self.height > ball.y && self.y1 + self.height < ball.y + ball.r;
        let up_side = self.y1 < ball.y + ball.r && self.y1 > ball.y;

        let ball_prev_frame: (f32, f32) = (ball.x - ball.x_speed, ball.y - ball.y_speed);

        match self.side {
             Side::Left => {
                if self.x1 + self.width > ball.x && self.x1 < ball.x && down_side {
                    let ball_corn = ((ball.y - ball_prev_frame.1) / (ball.x - ball_prev_frame.0)).atan();
                    let in_corn = ((ball.y - self.y1 - self.height) / (ball.x - self.x1 - self.width)).atan();

                    if ball_corn < in_corn { ball.x_speed *= -1.0; }
                    if ball_corn > in_corn { ball.y_speed *= -1.0; }
                    if ball_corn == in_corn { ball.x_speed *= -1.0; ball.y_speed *= -1.0; }

                    return Ok(());
                }

                if self.x1 + self.width > ball.x && self.x1 < ball.x && up_side {
                    let ball_corn = ((ball.y - ball_prev_frame.1) / (ball.x - ball_prev_frame.0)).atan();
                    let in_corn = ((self.y1 - (ball.y + ball.r)) / (self.x1 + self.width - ball.x)).atan();

                    if ball_corn > in_corn { ball.x_speed *= -1.0; }
                    if ball_corn < in_corn { ball.y_speed *= -1.0; }
                    if ball_corn == in_corn { ball.x_speed *= -1.0; ball.y_speed *= -1.0; }

                    return Ok(());
                }

                return Err(());
            },
            Side::Right => {
                if self.x1 < ball.x + ball.r && self.x1 + self.width > ball.x + ball.r && down_side {
                    let ball_corn = ((ball.y - ball_prev_frame.1) / (ball.x - ball_prev_frame.0)).atan();
                    let in_corn = ((ball.y - self.y1 - self.height) / (ball.x - self.x1 - self.width)).atan();

                    if ball_corn > in_corn { ball.x_speed *= -1.0; }
                    if ball_corn < in_corn { ball.y_speed *= -1.0; }
                    if ball_corn == in_corn { ball.x_speed *= -1.0; ball.y_speed *= -1.0; }

                    return Ok(());
                }
                
                if self.x1 < ball.x + ball.r && up_side {
                    let ball_corn = ((ball.y - ball_prev_frame.1) / (ball.x - ball_prev_frame.0)).atan();
                    let in_corn = ((self.y1 - (ball.y + ball.r)) / (self.x1 + self.width - ball.x)).atan();

                    if ball_corn < in_corn { ball.x_speed *= -1.0; }
                    if ball_corn > in_corn { ball.y_speed *= -1.0; }
                    if ball_corn == in_corn { ball.x_speed *= -1.0; ball.y_speed *= -1.0; }
                    
                    return Ok(());
                }

                return Err(());
            },
            _ => return Err(()),
        }
    }

    pub fn collision(&self, ball: &mut Ball) {
        let result = self.corner_collision(ball);
        match result {
            Ok(()) => (),
            Err(()) => self.direct_collision(ball),
        }
    }

    pub fn bot_move(&mut self, ball: &Ball, window_y: f32) {
        let speed = ball.y_speed.abs() * 0.75;
        if self.bot {
            match ball.y_speed == ball.y_speed.abs() {
                true => self.move_down(speed, window_y),
                false => self.move_up(speed),
            }
        }
    }
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

impl Score {
    pub fn new() -> Self {
        Self { p1 : 0, p2 : 0 }
    }
    
    pub fn add_score(&mut self, side: Side) {
        match side {
            Side::Left => self.p1 += 1,
            Side::Right => self.p2 += 1,
            _ => (),
        }
    }
}

struct MyGame {
    player1: Player,
    player2: Player,
    ball: Ball,
    score: Score,
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
        }
    }

    pub fn next_round(&mut self, window_size: (f32, f32)) {
        self.player1 = Player::new(Side::Left, window_size, true).unwrap();
        self.player2 = Player::new(Side::Right, window_size, false).unwrap();
        self.ball = Ball::new(window_size);
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
            },
            _ => (),
        }

        self.player1.collision(&mut self.ball);
        self.player2.collision(&mut self.ball);
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
        canvas.draw(&scores, DrawParam::default().dest([window_x / 2.0 - scores.glyph_positions(ctx).unwrap()[0].x as f32 / 2.0, window_y / 10.0]));
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
