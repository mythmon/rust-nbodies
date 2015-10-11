extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate num;

mod vector;
mod particle;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::context::Context;
use graphics::types::{Rectangle};

use self::vector::Vec2;
use self::particle::Particle;

struct Game {
    gl: GlGraphics,
    particles: Vec<Particle>,
    bounds: Rectangle,
    prev_dt: Option<f64>,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        let particles = self.particles.iter();
        self.gl.draw(args.viewport(), |context: Context, gl: &mut GlGraphics| {
            graphics::clear(graphics::color::BLACK, gl);

            for particle in particles {
                particle.render(args, context, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let ctx = UpdateContext {
            dt: args.dt,
            prev_dt: match self.prev_dt {
                Some(prev_dt) => prev_dt,
                None => args.dt,
            },
        };

        let center = Vec2::new(self.bounds[2] / 2.0, self.bounds[3] / 2.0);
        let center_mass = 10000.0;
        let big_g = 100.0;
        let particle_mass = 10.0;

        for mut p in &mut self.particles {
            let dist_vec = center - p.pos;
            let dist_squared = dist_vec.length_squared();
            let grav_mag = big_g * center_mass * particle_mass * dist_squared.recip();
            let grav = dist_vec.normal() * grav_mag;
            p.apply_force(grav);
            p.update(&ctx);
        }
    }
}

struct UpdateContext {
    dt: f64,
    prev_dt: f64,
}

trait Render {
    fn render(&self, args: &RenderArgs, ctx: Context, gl: &mut GlGraphics);
}

trait Update {
    fn update(&mut self, context: &UpdateContext);
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let width = 800;
    let height = 800;

    // Create an Glutin window.
    let window: Window = WindowSettings::new(
            "rustgame",
            [width, height]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        bounds: [0.0, 0.0, width as f64, height as f64],
        prev_dt: None,
        particles: vec![
            Particle::new(
                [1.0, 0.0, 0.0, 1.0],
                Vec2::new(20.0, height as f64 / 2.0),
                Vec2::new(0.0, 1.0),
            ),
        ],
    };

    for e in window.events() {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
        }
    }
}
