extern crate graphics;

use std::collections::HashSet;

use piston::event_loop::*;
use piston::input;
use opengl_graphics::GlGraphics;
use graphics::context::Context;

use super::camera::Camera;

pub struct UpdateContext<'a> {
    pub dt: f64,
    pub prev_dt: f64,
    pub buttons: &'a HashSet<input::Button>,
}

pub trait GameObject {
    fn render(&self, ctx: Context, gl: &mut GlGraphics);
    fn update(&mut self, context: &UpdateContext);
}

pub struct Game {
    gl: GlGraphics,
    children: Vec<Box<GameObject>>,
    prev_dt: Option<f64>,
    camera: Camera,
    buttons: HashSet<input::Button>,
}

impl Game {
    pub fn new(gl: GlGraphics, camera: Camera) -> Game {
        Game {
            gl: gl,
            children: vec![],
            prev_dt: None,
            camera: camera,
            buttons: HashSet::new(),
        }
    }

    pub fn add_child<T: GameObject + 'static>(&mut self, child: T) {
        self.children.push(Box::new(child));
    }

    pub fn key_press(&mut self, key: input::Button) {
        self.buttons.insert(key);
    }

    pub fn key_release(&mut self, key: input::Button) {
        self.buttons.remove(&key);
    }

    pub fn render(&mut self, args: &input::RenderArgs) {
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

    pub fn update(&mut self, args: &input::UpdateArgs) {
        let ctx = UpdateContext {
            dt: args.dt,
            prev_dt: match self.prev_dt {
                Some(prev_dt) => prev_dt,
                None => args.dt,
            },
            buttons: &self.buttons,
        };

        self.camera.update(&ctx);

        for mut p in &mut self.children {
            p.update(&ctx);
        }
    }
}
