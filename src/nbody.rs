extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate num;

// use num::Zero;
use piston::input::*;
use opengl_graphics::{GlGraphics};
use graphics::context::Context;

use super::particle::Particle;
// use super::vector::Vec2;
use super::game::{GameObject, UpdateContext};

#[derive(Debug, Clone)]
pub struct NBody {
    particles: Vec<Particle>,
    gravitation: f64,
}

impl NBody {
    pub fn new(particles: Vec<Particle>) -> NBody {
        NBody {
            particles: particles,
            gravitation: 5e4,
        }
    }
}

impl GameObject for NBody {
    fn render(&self, ctx: Context, gl: &mut GlGraphics) {
        for particle in self.particles.iter() {
            particle.render(ctx, gl);
        }
    }

    fn update(&mut self, ctx: &UpdateContext) {
        for i in 0..(self.particles.len() - 1) {
            for j in (i + 1)..self.particles.len() {
                let (mut left, mut right) = self.particles.split_at_mut(j);
                let ref mut p1 = left[i];
                let ref mut p2 = right[0];

                let dist_vec = p1.pos - p2.pos;

                // Gravity
                let dist_squared = dist_vec.length_squared().max((p1.radius + p2.radius).powi(2));
                let grav_mag = self.gravitation * p1.mass * p2.mass * dist_squared.recip();
                let grav = dist_vec.unit() * grav_mag;
                p1.apply_force(grav * -1.0);
                p2.apply_force(grav);

                // collision
                let collision_distance = p1.radius + p2.radius;
                let dist = dist_vec.length();
                if dist_vec.length() <= collision_distance {
                    let movement_dist = (dist - collision_distance) / 2.0;
                    let movement_vec = dist_vec.unit() * movement_dist;
                    p1.pos = p1.pos - movement_vec;
                    p2.pos = p2.pos + movement_vec;
                }
            }
        }

        for ref mut particle in &mut self.particles {
            particle.update(ctx);
        }
    }
}
