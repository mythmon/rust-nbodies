use graphics::Transformed;

use super::vector::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    trans: Vec2,
    zoom: f64,
}

impl Camera {
    pub fn new(trans: Vec2, zoom: f64) -> Camera {
        Camera { trans: trans, zoom: zoom }
    }

    pub fn apply<T: Transformed>(&self, transform: T) -> T {
        transform.trans(self.trans.x, self.trans.y).zoom(self.zoom)
    }
}
