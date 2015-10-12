extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate num;

use opengl_graphics::{GlGraphics};
use graphics::context::Context;
use graphics::types::{Color};
use graphics::Transformed;
use num::traits::Zero;

use super::vector::Vec2;
use super::{GameObject, UpdateContext};

#[derive(Debug, Clone)]
pub struct Particle {
    pub color: Color,
    pub pos: Vec2,
    pub mass: f64,
    prev_pos: Vec2,
    accel: Vec2,
    radius: f64,
}

impl Particle {
    pub fn new(color: Color, pos: Vec2, vel: Vec2, mass: f64) -> Particle {
        Particle {
            color: color,
            pos: pos,
            prev_pos: pos - vel,
            accel: Vec2::zero(),
            mass: mass,
            radius: mass.sqrt(),
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.accel = self.accel + force / self.mass;
    }
}

impl GameObject for Particle {
    fn render(&self, ctx: Context, gl: &mut GlGraphics) {
        let transform = ctx.transform.trans(self.pos.x, self.pos.y);
        let bounding_box = [-self.radius, -self.radius, self.radius * 2.0, self.radius * 2.0];
        graphics::ellipse(self.color, bounding_box, transform, gl);
    }

    fn update(&mut self, ctx: &UpdateContext) {
        let cur_pos = self.pos;
        self.pos = cur_pos +
            (cur_pos - self.prev_pos) * ctx.dt / ctx.prev_dt +
            self.accel * (ctx.dt + ctx.prev_dt) / 2.0 * ctx.dt;
        self.prev_pos = cur_pos;
        self.accel = Vec2::zero();
    }
}
