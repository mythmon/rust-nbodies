extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate num;

use std::collections::VecDeque;

use opengl_graphics::{GlGraphics};
use graphics::Transformed;
use graphics::context::Context;
use graphics::types::{Color};

use super::vector::Vec2;
use super::{GameObject, UpdateContext};

#[derive(Debug, Clone)]
pub struct Trail {
    color: Color,
    points: VecDeque<Vec2>,
    max_size: usize,
}

impl Trail {
    pub fn new(color: Color, max_size: usize) -> Trail {
        Trail {
            color: color,
            max_size: max_size,
            points: VecDeque::with_capacity(max_size),
        }
    }

    pub fn push(&mut self, new_point: Vec2) {
        self.points.push_front(new_point);
        while self.points.len() > self.max_size {
            self.points.pop_back();
        }
    }
}

impl GameObject for Trail {
    fn render(&self, ctx: Context, gl: &mut GlGraphics) {
        // for window in self.points.windows(2) {
        //     let p1 = window[0];
        //     let p2 = window[1];
        //     let diff = p2 - p1;
        //     let transform = ctx.transform.orient(diff.x, diff.y);
        //     let rect = [0.0, -2.0, diff.length(), 4.0];
        //     graphics::rectangle(self.color, rect, transform, gl);
        // }
        for p in self.points.iter() {
            let transform = ctx.transform.trans(p.x, p.y);
            graphics::rectangle(self.color, [0.0, 0.0, 1.0, 1.0], transform, gl);
        }
    }

    fn update(&mut self, _: &UpdateContext) {
        // do nothing
    }
}
