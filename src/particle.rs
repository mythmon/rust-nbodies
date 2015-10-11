extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate num;

use piston::input::*;
use opengl_graphics::{GlGraphics};
use graphics::context::Context;
use graphics::types::{Color};
use num::traits::Zero;

use super::vector::Vec2;
use super::{Update, Render, UpdateContext};

pub struct Particle {
    pub color: Color,
    pub pos: Vec2,
    prev_pos: Vec2,
    accel: Vec2,
}

impl Particle {
    pub fn new(color: Color, pos: Vec2, vel: Vec2) -> Particle {
        Particle {
            color: color,
            pos: pos,
            prev_pos: pos - vel,
            accel: Vec2::zero(),
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.accel = self.accel + force;
    }
}

impl Render for Particle {
    fn render(&self, _: &RenderArgs, ctx: Context, gl: &mut GlGraphics) {
        use graphics::Transformed;
        let transform = ctx.transform
            .trans(self.pos.x, self.pos.y);
        graphics::ellipse(self.color, [0.0, 0.0, 10.0, 10.0], transform, gl);
    }
}

impl Update for Particle {
    fn update(&mut self, ctx: &UpdateContext) {
        let cur_pos = self.pos;
        self.pos = cur_pos +
            (cur_pos - self.prev_pos) * ctx.dt / ctx.prev_dt +
            self.accel * (ctx.dt + ctx.prev_dt) / 2.0 * ctx.dt;
        self.prev_pos = cur_pos;
        self.accel = Vec2::zero();
    }
}
