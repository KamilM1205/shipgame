pub mod game_state;
pub mod menu_state;

use async_trait::async_trait;

pub struct StateRouter<'a> {
    state: &'a mut dyn State,
}

impl<'a> StateRouter<'a> {
    pub async fn new(state: &'a mut dyn State) -> StateRouter<'a> {
        state.load().await;
        Self { state }
    }

    pub async fn _set_state(&mut self, state: &'a mut dyn State) {
        self.state = state;
        self.state.load().await;
    }

    pub async fn update(&mut self, dt: f32) {
        self.state.update(dt).await;
    }

    pub async fn draw(&self) {
        self.state.draw().await
    }
}

#[async_trait]
pub trait State {
    async fn load(&mut self);
    async fn update(&mut self, dt: f32);
    async fn draw(&self);
}
