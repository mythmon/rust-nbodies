extern crate num;

use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x: x, y: y }
    }

    pub fn normal(&self) -> Vec2 {
        let len = self.length();
        Vec2::new(self.x / len, self.y / len)
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }
}

impl num::Zero for Vec2 {
    fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }

    fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> ops::Mul<T> for Vec2
    where f64: ops::Mul<T, Output=f64>,
          T: Copy,
{
    type Output = Vec2;

    fn mul(self, rhs: T) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        rhs * self
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Vec2 {
        self * rhs.recip()
    }
}
