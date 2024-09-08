use std::ops::{Add, Mul, Sub};

///
/// # Fluent Polygon
/// support any fluent polygon
/// Notice: your draw can draw any shape depended by an polygon

use crate::fluent::Fluent;
use macroquad::prelude::*;

/// draw a shape depended by an polygon
pub type Drawable = dyn Fn(&Vec<Vec2>);

#[derive(Clone)]
pub struct Polygon {
    points: Vec<Vec2>,
}

impl Add for Polygon {
    type Output = Polygon;

    fn add(self, other: Self) -> Self::Output {
        assert_eq!(self.points.len(), other.points.len());
        let points = self
            .points
            .into_iter()
            .zip(other.points.into_iter())
            .map(|(a, b)| a + b)
            .collect();
        Polygon { points }
    }
}

impl Sub for Polygon {
    type Output = Polygon;

    fn sub(self, other: Self) -> Self::Output {
        assert_eq!(self.points.len(), other.points.len());
        let points = self
            .points
            .into_iter()
            .zip(other.points.into_iter())
            .map(|(a, b)| a - b)
            .collect();
        Polygon { points }
   }
}

impl Mul<f32> for Polygon {
    type Output = Polygon;

    fn mul(self, rhs: f32) -> Self::Output {
        let points = self.points.into_iter().map(|p| p * rhs).collect();
        Polygon { points }
    }
}

impl Polygon {
    pub fn new(points: Vec<Vec2>) -> Self {
        Self { points }
    }
}

pub struct FluentPolygon {
    points: Fluent<Polygon>,
    texture: Box<Drawable>,
}

impl FluentPolygon {
    pub fn new(points: Polygon, texture: Box<Drawable>, duration_secs: f32) -> Self {
        Self {
            points: Fluent::new(points, duration_secs),
            texture,
        }
    }

    pub fn set_target(&mut self, target_points: Polygon) {
        self.points.set_target(target_points);
    }

    pub fn draw(&self) {
        (self.texture)(&self.points.value.points);
    }
}