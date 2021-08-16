use crate::game_objects::object::GameObject;

use async_trait::async_trait;

use macroquad::color::WHITE;
use macroquad::texture::{draw_texture, load_texture, Texture2D};

#[derive(Copy, Clone)]
pub struct Tile {
    x: f32,
    y: f32,
    speed: f32,
    txt: Texture2D,
    is_visible: bool,
}

impl Tile {
    pub async fn new(path: &str) -> Self {
        let txt = load_texture(path).await.unwrap();
        Self {
            x: 0.,
            y: 0.,
            speed: 0.,
            txt,
            is_visible: false,
        }
    }
    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y
    }
}

#[async_trait]
impl GameObject for Tile {
    async fn load(&mut self) {}
    async fn update(&mut self, dt: f32) {
        self.y += self.speed * dt;
        if self.y + self.txt.width() > 0. && self.y < 480. {
            self.is_visible = true
        } else {
            self.is_visible = false
        }
    }
    async fn draw(&self) {
        if self.is_visible {
            draw_texture(self.txt, self.x, self.y, WHITE)
        }
    }
}
