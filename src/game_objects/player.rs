use crate::game_objects::bullet::Bullet;
use crate::game_objects::explosion::Explosion;
use crate::game_objects::object::GameObject;

use async_trait::async_trait;

use macroquad::color::{Color, WHITE};
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::texture::{draw_texture, load_texture, FilterMode, Texture2D};

pub enum Bufs {}

enum BulletPos {
    Left,
    Right,
}

pub struct Player {
    x: f32,
    y: f32,
    speed: f32,
    hp: f32,
    armor: f32,
    buf: Option<Bufs>,
    txt: Texture2D,
    shadow: Texture2D,
    is_visible: bool,
    expl: Explosion,
    bullets: Vec<Bullet>,
    bullet_pos: BulletPos,
    bullet_counter: usize,
    fire_timer: f32,
    fire_timer_speed: f32,
}

impl Player {
    pub async fn new() -> Self {
        let txt = load_texture("assets/Ships/ship_0002.png").await.unwrap();
        txt.set_filter(FilterMode::Nearest);
        let shadow = txt.clone();
        let bullet = Bullet::new().await;
        let mut bullets: Vec<Bullet> = Vec::new();
        for _ in 1..10 {
            bullets.push(bullet.clone())
        }
        Self {
            x: 0.,
            y: 0.,
            speed: 500.,
            
            hp: 100.,
            armor: 0.,
            buf: None,
            txt,
            shadow,
            is_visible: true,
            expl: Explosion::new().await,
            bullets,
            bullet_pos: BulletPos::Left,
            bullet_counter: 0,
            fire_timer: 0.,
            fire_timer_speed: 5.,
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn move_pl(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    pub fn destroy(&mut self) {
        self.is_visible = false;
        self.hit(0.)
    }

    pub fn hit(&mut self, dmg: f32) {
        self.hp -= dmg;
        self.expl.set_pos(
            self.x + self.txt.width() / 2. - 8.,
            self.y + self.txt.height() / 2. - 8.,
        );
        self.expl.get_anim().set_visible(true);
        self.expl.get_anim().play()
    }

    fn fire(&mut self) {
        if self.bullet_counter >= self.bullets.len() {
            self.bullet_counter = 0
        }

        match self.bullet_pos {
            BulletPos::Left => {
                self.bullet_pos = BulletPos::Right;
                let bullet = self.bullets.get_mut(self.bullet_counter).unwrap();
                bullet.move_to(self.x + self.txt.width() - bullet.width(), self.y);
                self.bullet_counter += 1;
            }
            BulletPos::Right => {
                self.bullet_pos = BulletPos::Left;
                let bullet = self.bullets.get_mut(self.bullet_counter).unwrap();
                bullet.move_to(self.x, self.y);
                self.bullet_counter += 1;
            }
        }
    }
}

#[async_trait]
impl GameObject for Player {
    async fn load(&mut self) {
        self.set_pos(
            640. / 2. - self.txt.width() / 2.,
            480. * 0.75 + self.txt.height() / 2.,
        );
        self.expl.load().await;
    }
    async fn update(&mut self, dt: f32) {
        if self.hp <= 0. && self.is_visible {
            self.destroy()
        }

        if self.x > 640. {
            self.x = 640. - self.txt.width();
        }
        if self.y > 480. {
            self.y = 480. - self.txt.height();
        }
        if self.x < 0. {
            self.x = 0.
        }
        if self.y < 240. {
            self.y = 240.
        }

        self.expl.update(dt).await;
        for i in self.bullets.iter_mut() {
            i.update(dt).await
        }
        self.fire_timer += self.fire_timer_speed * dt;

        if is_key_pressed(KeyCode::R) {
            self.is_visible = true;
            self.hp = 100.
        }

        if !self.is_visible {
            return;
        }

        if is_key_pressed(KeyCode::E) {
            self.hit(10.)
        }

        if self.fire_timer > 1. {
            self.fire();
            self.fire_timer = 0.
        }

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            if self.y > 240. {
                self.move_pl(0., -self.speed * dt)
            }
        } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            if self.y + self.txt.height() < 480. {
                self.move_pl(0., self.speed * dt)
            }
        } else if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            if self.x > 0. {
                self.move_pl(-self.speed * dt, 0.)
            }
        } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            if self.x + self.txt.width() < 640. {
                self.move_pl(self.speed * dt, 0.)
            }
        }
    }
    async fn draw(&self) {
        if self.is_visible {
            draw_texture(
                self.shadow,
                self.x - self.txt.width() / 2. + 4.,
                self.y + self.txt.height() / 2. - 4.,
                Color::new(0., 0., 0., 0.4),
            );
            draw_texture(self.txt, self.x, self.y, WHITE);
            for i in self.bullets.iter() {
                i.draw().await
            }
        }
        self.expl.draw().await;
    }
}
