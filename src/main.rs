extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate num;
extern crate rand;

mod vector;
mod particle;
mod nbody;
mod trail;
mod camera;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::context::Context;
use rand::Rng;

use self::vector::Vec2;
use self::particle::Particle;
use self::nbody::NBody;
use self::camera::Camera;

struct Game {
    gl: GlGraphics,
    children: Vec<Box<GameObject>>,
    prev_dt: Option<f64>,
    camera: Camera,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        let particles = self.children.iter();
        let camera = self.camera;
        self.gl.draw(args.viewport(), |context: Context, gl: &mut GlGraphics| {
            graphics::clear(graphics::color::BLACK, gl);
            let mut context = context;
            context.transform = camera.apply(context.transform);

            for particle in particles {
                particle.render(context, gl);
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

        for mut p in &mut self.children {
            p.update(&ctx);
        }
    }
}

struct UpdateContext {
    dt: f64,
    prev_dt: f64,
}

trait GameObject {
    fn render(&self, ctx: Context, gl: &mut GlGraphics);
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

    let mut particles = vec![];
    let mut rng = rand::thread_rng();

    for _ in 0..4 {
        particles.push(Particle::new(
            [rng.gen_range(0.5, 1.0), rng.gen_range(0.5, 1.0), rng.gen_range(0.5, 1.0), 1.0],
            Vec2::new(rng.gen_range(-400.0, 400.0), rng.gen_range(-400.0, 400.0)),
            Vec2::new(rng.gen_range(-2.0, 2.0), rng.gen_range(-2.0, 2.0)),
            // Vec2::zero(),
            rng.gen_range(30.0, 200.0),
        ));
    }

    let nbody = NBody::new(particles);

    // Create a new game and run it.
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        prev_dt: None,
        children: vec![Box::new(nbody)],
        camera: Camera::new(Vec2::new(400.0, 400.0), 0.1),
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
