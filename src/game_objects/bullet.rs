use crate::game_objects::object::GameObject;

use async_trait::async_trait;

use macroquad::color::WHITE;
use macroquad::texture::{draw_texture, load_texture, Texture2D};

#[derive(Copy, Clone)]
pub struct Bullet {
    x: f32,
    y: f32,
    speed: f32,
    is_alive: bool,
    damage: f32,
    txt: Texture2D,
}

#[allow(dead_code)]
impl Bullet {
    pub async fn new() -> Self {
        let txt = load_texture("assets/Tiles/tile_0000.png").await.unwrap();
        Self {
            x: 0.,
            y: 0.,
            speed: 1000.,
            is_alive: false,
            damage: 5.,
            txt,
        }
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.is_alive = true;
        self.x = x;
        self.y = y;
    }

    pub fn width(self) -> f32 {
        self.txt.width()
    }

    pub fn height(self) -> f32 {
        self.txt.height()
    }
}

#[async_trait]
impl GameObject for Bullet {
    async fn load(&mut self) {}
    async fn update(&mut self, dt: f32) {
        if !self.is_alive {
            return;
        }

        if self.y > 0. - self.txt.height() {
            self.y -= self.speed * dt
        } else {
            self.is_alive = false
        }
    }
    async fn draw(&self) {
        if self.is_alive {
            draw_texture(self.txt, self.x, self.y, WHITE)
        }
    }
}
