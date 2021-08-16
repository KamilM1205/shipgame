use async_trait::async_trait;

#[async_trait]
pub trait GameObject {
    async fn load(&mut self);
    async fn update(&mut self, dt: f32);
    async fn draw(&self);
}
