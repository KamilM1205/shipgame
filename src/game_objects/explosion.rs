use crate::game_objects::animation::Animation;
use crate::game_objects::object::GameObject;

use async_trait::async_trait;

pub struct Explosion {
    anim: Animation,
}

impl Explosion {
    pub async fn new() -> Self {
        let files = vec![
            "assets/Tiles/tile_0004.png",
            "assets/Tiles/tile_0005.png",
            "assets/Tiles/tile_0006.png",
            "assets/Tiles/tile_0007.png",
            "",
        ];
        let anim = Animation::new(files, 80., 80., 15.).await;
        Self { anim }
    }

    pub fn get_anim(&mut self) -> &mut Animation {
        &mut self.anim
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.anim.set_pos(x, y)
    }
}

#[async_trait]
impl GameObject for Explosion {
    async fn load(&mut self) {}
    async fn update(&mut self, dt: f32) {
        self.anim.update(dt).await
    }
    async fn draw(&self) {
        self.anim.draw().await
    }
}
