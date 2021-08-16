use crate::game_objects::object::GameObject;
use crate::utils::lvl_loader::Level;

use async_trait::async_trait;

pub struct Ground {
    level: Level,
    speed: f32,
}

impl Ground {
    pub async fn new() -> Self {
        Self {
            level: Level::load("lvl1.json").await,
            speed: 1.,
        }
    }
}

#[async_trait]
impl GameObject for Ground {
    async fn load(&mut self) {
        self.level.set_pos(0., -(self.level.map.layers["main"].height as f32) * 16. + 30. * 16.)
    } 
    async fn update(&mut self, dt: f32) {
        let (x, y) = self.level.get_pos();
        if y < 0. {
            self.level.set_pos(x, y + self.speed)
        } else {
            self.level.set_pos(x, 0.)
        }
    }
    async fn draw(&self) {
        self.level.draw().await;
    }
}
