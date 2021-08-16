use macroquad::texture::{draw_texture, load_texture, FilterMode, Texture2D};

pub struct Animation {
    texts: Vec<Option<Texture2D>>,
    curr_frame: usize,
    timer: f32,
    speed: f32,
    x: f32,
    y: f32,
    is_playing: bool,
    is_visible: bool,
    is_looped: bool,
}

#[allow(dead_code)]
impl Animation {
    pub async fn new(files: Vec<&str>, x: f32, y: f32, speed: f32) -> Self {
        let mut texts: Vec<Option<Texture2D>> = Vec::new();

        for f in files {
            if f == "" {
                texts.push(None);
                continue;
            }
            texts.push(match load_texture(f).await {
                Ok(t) => {
                    t.set_filter(FilterMode::Nearest);
                    Some(t)
                }
                Err(e) => {
                    println!("{}", e);
                    None
                }
            })
        }
        Self {
            texts,
            curr_frame: 0,
            timer: 0.,
            speed,
            x,
            y,
            is_playing: false,
            is_visible: false,
            is_looped: false,
        }
    }

    pub async fn update(&mut self, dt: f32) {
        if !self.is_playing {
            return;
        }

        self.timer += self.speed * dt;
        if self.timer as usize > self.curr_frame && self.timer as usize <= self.texts.len() {
            self.curr_frame = self.timer as usize;
        }
        if self.curr_frame as usize >= self.texts.len() {
            self.timer = 0.;
            self.curr_frame = 0;
            if !self.is_looped {
                self.stop();
                self.set_visible(false)
            }
        }
    }

    pub async fn draw(&self) {
        if let Some(Some(txt)) = self.texts.get(self.curr_frame) {
            if self.is_visible {
                draw_texture(*txt, self.x, self.y, macroquad::color::WHITE)
            }
        }
    }

    pub fn play(&mut self) {
        self.is_playing = true
    }

    pub fn pause(&mut self) {
        self.is_playing = false
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.timer = 0.;
        self.curr_frame = 0;
    }

    pub fn set_visible(&mut self, is_visible: bool) {
        self.is_visible = is_visible
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}
