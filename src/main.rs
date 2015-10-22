extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate num;
extern crate rand;

mod vector;
mod particle;
mod nbody;
mod camera;
mod game;

use std::f64::consts::PI;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::{Input, Event};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use rand::Rng;

use self::vector::Vec2;
use self::particle::Particle;
use self::nbody::NBody;
use self::camera::Camera;
use self::game::Game;

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

    for _ in 0..100 {
        let theta = rng.gen_range(0.0, 2.0 * PI);
        let dir = Vec2::new(theta.cos(), theta.sin());
        let distance: f64 = rng.gen_range(100.0, 4500.0);
        let mass = rng.gen_range(100.0, 200.0);
        let momentum = rng.gen_range(1.0, 4.0) * distance.sqrt();
        let speed = momentum / mass;

        particles.push(Particle::new(
            [rng.gen_range(0.5, 1.0), rng.gen_range(0.5, 1.0), rng.gen_range(0.5, 1.0), 1.0],
            dir * distance,
            dir.normal() * speed,
            mass,
        ));
    }

    // Create a new game and run it.
    let camera = Camera::new(Vec2::new(400.0, 400.0), 0.05);
    let mut game = Game::new(GlGraphics::new(opengl), camera);
    game.add_child(NBody::new(particles));

    for e in window.events().ups(60).max_fps(60) {

        match e {
            Event::Input(Input::Press(button)) => {
                game.key_press(button);
            }

            Event::Input(Input::Release(button)) => {
                game.key_release(button);
            }

            Event::Render(args) => {
                game.render(&args);
            }

            Event::Update(args) => {
                game.update(&args);
            }

            _ => {}
        }
    }
}
