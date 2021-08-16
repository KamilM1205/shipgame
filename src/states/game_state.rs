use crate::game_objects::ground::Ground;
use crate::game_objects::object::GameObject;
use crate::game_objects::player::Player;
use crate::State;

use macroquad::color::SKYBLUE;
use macroquad::window::clear_background;

use async_trait::async_trait;

pub struct Game {
    player: Player,
    ground: Ground,
}

impl Game {
    pub async fn new() -> Self {
        let player = Player::new().await;
        let ground = Ground::new().await;
        Self {player, ground }
    }
}

#[async_trait]
impl State for Game {
    async fn load(&mut self) {
        self.player.load().await;
        self.ground.load().await;
    }
    async fn update(&mut self, dt: f32) {
        self.player.update(dt).await;
        self.ground.update(dt).await;
    }
    async fn draw(&self) {
        clear_background(SKYBLUE);
        self.ground.draw().await;
        self.player.draw().await;
    }
}
