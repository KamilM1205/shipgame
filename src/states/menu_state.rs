use crate::State;

use macroquad::color::*;
use macroquad::window::*;

use async_trait::async_trait;

pub struct Menu;

#[async_trait]
impl State for Menu {
    async fn load(&mut self) {}
    async fn update(&mut self, _: f32) {}
    async fn draw(&self) {
        clear_background(DARKBLUE)
    }
}
