extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate num;

use piston::input::*;
use opengl_graphics::{GlGraphics};
use graphics::context::Context;

use super::particle::Particle;
use super::trail::Trail;
use super::{GameObject, UpdateContext};

#[derive(Debug, Clone)]
pub struct NBody {
    particles: Vec<(Particle, Trail)>,
    gravitation: f64,
}

impl NBody {
    pub fn new(particles: Vec<Particle>) -> NBody {
        NBody {
            particles: particles.into_iter().map(|p: Particle| {
                let mut trail_color = p.color;
                trail_color[3] = 0.3;
                let mut trail = Trail::new(trail_color, 1000);
                trail.push(p.pos);
                (p, trail)
            }).collect(),
            gravitation: 1e5,
        }
    }
}

impl GameObject for NBody {
    fn render(&self, ctx: Context, gl: &mut GlGraphics) {
        for &(ref particle, ref trail) in self.particles.iter() {
            particle.render(ctx, gl);
            trail.render(ctx, gl);
        }
    }

    fn update(&mut self, ctx: &UpdateContext) {
        for i in 0..(self.particles.len() - 1) {
            for j in (i + 1)..self.particles.len() {
                let grav = {
                    let ref p1 = self.particles[i].0;
                    let ref p2 = self.particles[j].0;

                    let dist_vec = p1.pos - p2.pos;
                    let mut dist_squared = dist_vec.length_squared();
                    if dist_squared < 10000.0 {
                        dist_squared = 10000.0;
                    }
                    let grav_mag = self.gravitation * p1.mass * p2.mass * dist_squared.recip();
                    dist_vec.normal() * grav_mag
                };

                self.particles[i].0.apply_force(grav * -1.0);
                self.particles[j].0.apply_force(grav);
            }
        }

        for &mut (ref mut particle, ref mut trail) in &mut self.particles {
            particle.update(ctx);
            trail.push(particle.pos);
        }
    }
}
