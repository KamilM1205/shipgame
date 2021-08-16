use macroquad::camera::{set_camera, Camera2D};
use macroquad::color::BLACK;
use macroquad::math::Rect;
use macroquad::text::draw_text;
use macroquad::time::{get_fps, get_frame_time};
use macroquad::window::{next_frame, Conf};

mod config;
mod game_objects;
mod states;
mod utils;
//mod ui;

use self::states::{game_state, State, StateRouter};

fn configure_window() -> Conf {
    Conf {
        window_title: "ShipGame".to_string(),
        window_width: 640,
        window_height: 480,
        //window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(configure_window)]
async fn main() {
    let camera = Camera2D::from_display_rect(Rect::new(0., 0., 640., 480.));
    set_camera(&camera);
    
    let mut game = game_state::Game::new().await;
    //let menu = menu_state::Menu {};
    let mut router = StateRouter::new(&mut game).await;
    let mut dt: f32;
    let mut fps: i32;
    loop {
        dt = get_frame_time();
        fps = get_fps();
        router.update(dt).await;
        router.draw().await;
        draw_text(&format!("fps: {}", fps), 10., 15., 24., BLACK);
        next_frame().await
    }
}
