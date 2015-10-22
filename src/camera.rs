extern crate graphics;

use graphics::context::Context;
use graphics::Transformed;
use opengl_graphics::GlGraphics;
use piston::input::{Button, Key};
use num::Zero;

use super::vector::Vec2;
use super::game::{GameObject, UpdateContext};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    trans: Vec2,
    zoom: f64,
    trans_vel: Vec2,
    zoom_vel: f64,
}

impl Camera {
    pub fn new(trans: Vec2, zoom: f64) -> Camera {
        Camera {
            trans: trans,
            zoom: zoom,
            trans_vel: Vec2::zero(),
            zoom_vel: 0.0,
        }
    }

    pub fn apply<T: Transformed>(&self, transform: T) -> T {
        transform.trans(self.trans.x, self.trans.y).zoom(self.zoom)
    }
}

impl GameObject for Camera {
    fn render(&self, _: Context, _: &mut GlGraphics) {}

    fn update(&mut self, ctx: &UpdateContext) {
        macro_rules! if_key {
            ($key:path : $ctx:ident $then:block) => {
                if $ctx.buttons.contains(&Button::Keyboard($key)) {
                    $then
                }
            };
        }

        let scroll_speed = 0.7;
        if_key! [ Key::Up : ctx { self.trans_vel = self.trans_vel + Vec2::new(0.0, scroll_speed); }];
        if_key! [ Key::Down : ctx { self.trans_vel = self.trans_vel + Vec2::new(0.0, -scroll_speed); }];
        if_key! [ Key::Left : ctx { self.trans_vel = self.trans_vel + Vec2::new(scroll_speed, 0.0); }];
        if_key! [ Key::Right : ctx { self.trans_vel = self.trans_vel + Vec2::new(-scroll_speed, 0.0); }];

        let zoom_amount = 0.005;
        if_key! [ Key::PageUp : ctx { self.zoom_vel += zoom_amount; }];
        if_key! [ Key::PageDown : ctx { self.zoom_vel -= zoom_amount; }];

        self.trans = self.trans + self.trans_vel;
        self.trans_vel = self.trans_vel * 0.9;
        self.zoom *= 1.0 + self.zoom_vel;
        self.zoom_vel *= 0.9;
    }
}
