use crate::game::Side;
use rand::Rng;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub x_speed: f32,
    pub y_speed: f32,
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
