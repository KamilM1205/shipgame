use macroquad::file::load_string;
use macroquad::math::Rect;
use macroquad::texture::{load_texture, FilterMode};

use macroquad_tiled as tiled;

pub struct Level<'a> {
    x: f32,
    y: f32,
    pub map: tiled::Map,
    tiles: Vec<&'a Option<tiled::Tile>>
}

impl<'a> Level<'a> {
    pub async fn load(lvl_name: &str) -> Self {
        let tileset = load_texture("assets/Tilemap/tiles.png").await.unwrap();
        tileset.set_filter(FilterMode::Nearest);

        let tiled_map_json = load_string(&("assets/levels/".to_string() + lvl_name))
            .await
            .unwrap();
        let tiled_tileset = load_string("assets/levels/ShipGame.json").await.unwrap();
        let tiled_map = tiled::load_map(
            &tiled_map_json,
            &[("assets/Tilemap/tiles.png", tileset)],
            &[("ShipGame.json", &tiled_tileset)],
        )
        .unwrap();

        Self { x: 0., y: 0., map: tiled_map, tiles: vec!(&'a one) }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub async fn update(&mut self) {
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        for lay_name in self.map.layers.keys() {
            while y < self.map.layers[lay_name].height {
                while x < self.map.layers[lay_name].width {
                    self.map.get_tile(layer: &str, x: u32, y: u32)
                }
            }
        }
    }

    pub async fn draw(&self) {
        for lay_name in self.map.layers.keys() {
            self.map.draw_tiles(
                lay_name,
                Rect::new(
                    self.x,
                    self.y,
                    self.map.layers[lay_name].width as f32 * 16.,
                    self.map.layers[lay_name].height as f32 * 16.,
                ),
                None,
            );
        }
    }
}
