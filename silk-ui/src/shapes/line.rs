use macroquad::prelude::*;
use crate::fluent::Interpolatable;
use crate::shapes::Drawable;

pub struct Line {
    pos: Vec4,
    thickness: f32,
    color: Color,
}

impl Line {
    fn from(a: Vec2, b: Vec2, thickness: f32, color: Color) -> Self {
        Self {
            pos: vec4(a.x, a.y, b.x, b.y),
            thickness,
            color,
        }
    }
}

impl From<Line> for Vec4 {
    fn from(value: Line) -> Self {
        value.pos
    }
}

impl Drawable for Line {
    fn draw(&self) {
        draw_line(self.pos.x, self.pos.y, self.pos.z, self.pos.w, self.thickness, self.color);
    }
}

impl Interpolatable for Line {
    fn interpolate(&self, other: &Self, progress: f32) -> Self {
        Self {
            pos: self.pos.interpolate(&other.pos, progress),
            thickness: self.thickness.interpolate(&other.thickness, progress),
            color: Color::from_vec(self.color.to_vec() + (other.color.to_vec() - self.color.to_vec()) * progress),
        }
    }
}