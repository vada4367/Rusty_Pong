use crate::game::Ball;
use crate::game::Side;

pub struct Player {
    pub x1: f32,
    pub y1: f32,
    pub width: f32,
    pub height: f32,
    side: Side,
    pub bot: bool,
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

    fn direct_collision(&self, ball: &mut Ball) -> Result<(), ()> {
        if ((self.side == Side::Left && self.x1 + self.width > ball.x)
            || (self.side == Side::Right && self.x1 < ball.x + ball.r))
            && (self.y1 < ball.y && self.y1 + self.height > ball.y + ball.r)
        {
            ball.x_speed *= -1.0;
            return Ok(());
        }
        Err(())
    }

    fn corner_collision(&self, ball: &mut Ball) -> Result<(), ()> {
        let down_side = self.y1 + self.height > ball.y && self.y1 + self.height < ball.y + ball.r;
        let up_side = self.y1 < ball.y + ball.r && self.y1 > ball.y;

        let ball_prev_frame: (f32, f32) = (ball.x - ball.x_speed, ball.y - ball.y_speed);

        match self.side {
            Side::Left => {
                if self.x1 + self.width > ball.x && self.x1 < ball.x && down_side {
                    let ball_corn =
                        ((ball_prev_frame.1 - ball.y) / (ball_prev_frame.0 - ball.x)).atan();
                    let in_corn =
                        ((self.y1 + self.height - ball.y) / (self.x1 + self.width - ball.x)).atan();

                    if ball_corn > in_corn {
                        ball.y_speed *= -1.0;
                    }
                    if ball_corn < in_corn {
                        ball.x_speed *= -1.0;
                    }
                    if ball_corn == in_corn {
                        ball.x_speed *= -1.0;
                        ball.y_speed *= -1.0;
                    }

                    return Ok(());
                }

                if self.x1 + self.width > ball.x && self.x1 < ball.x && up_side {
                    let ball_corn =
                        ((ball.y - ball_prev_frame.1) / (ball_prev_frame.0 - ball.x)).atan();
                    let in_corn =
                        ((ball.y + ball.r - self.y1) / (self.x1 + self.width - ball.x)).atan();

                    if ball_corn > in_corn {
                        ball.y_speed *= -1.0;
                    }
                    if ball_corn < in_corn {
                        ball.x_speed *= -1.0;
                    }
                    if ball_corn == in_corn {
                        ball.x_speed *= -1.0;
                        ball.y_speed *= -1.0;
                    }

                    return Ok(());
                }

                return Err(());
            }
            Side::Right => {
                if self.x1 < ball.x + ball.r && self.x1 + self.width > ball.x + ball.r && down_side
                {
                    let ball_corn =
                        ((ball_prev_frame.1 - ball.y) / (ball.x - ball_prev_frame.0)).atan();
                    let in_corn =
                        ((self.y1 + self.height - ball.y) / (ball.x + ball.r - self.x1)).atan();

                    if ball_corn > in_corn {
                        ball.y_speed *= -1.0;
                    }
                    if ball_corn < in_corn {
                        ball.x_speed *= -1.0;
                    }
                    if ball_corn == in_corn {
                        ball.x_speed *= -1.0;
                        ball.y_speed *= -1.0;
                    }

                    return Ok(());
                }

                if self.x1 < ball.x + ball.r && up_side {
                    let ball_corn =
                        ((ball.y - ball_prev_frame.1) / (ball.x - ball_prev_frame.0)).atan();
                    let in_corn =
                        ((ball.x + ball.r - self.y1) / (ball.x + ball.r - self.x1)).atan();

                    if ball_corn > in_corn {
                        ball.y_speed *= -1.0;
                    }
                    if ball_corn < in_corn {
                        ball.x_speed *= -1.0;
                    }
                    if ball_corn == in_corn {
                        ball.x_speed *= -1.0;
                        ball.y_speed *= -1.0;
                    }

                    return Ok(());
                }

                return Err(());
            }
            _ => return Err(()),
        }
    }

    pub fn collision(&self, ball: &mut Ball) -> Result<(), ()>{
        let corn_res = self.corner_collision(ball);
        let mut dir_res = Err(());

        match corn_res {
            Ok(()) => (),
            Err(()) => dir_res = self.direct_collision(ball),
        }

        if dir_res == Ok(()) || corn_res == Ok(()) {
            return Ok(());
        }

        Err(())
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
