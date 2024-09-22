use macroquad::prelude::*;
use crate::draw::draw_capsule;
use crate::fluent::Interpolatable;
use super::Drawable;

pub struct Capsule {
    left: f32,
    top: f32,
    width: f32,
    height: f32,
    color: Color,
}

impl Capsule {
    fn new(left: f32, top: f32, width: f32, height: f32, color: Color) -> Self {
        Self {
            left,
            top,
            width,
            height,
            color,
        }
    }

    fn center_in(&mut self, left: f32, top: f32) {
        self.left = left - self.width / 2.;
        self.top = top - self.height / 2.;
    }
}

impl Drawable for Capsule {
    fn draw(&self) {
        draw_capsule(self.left, self.top, self.width, self.height, self.color);
    }
}

impl From<(Rect, Color)> for Capsule {
    fn from((value, color): (Rect, Color)) -> Self {
        Self {
            left: value.x,
            top: value.y,
            width: value.w,
            height: value.h,
            color,
        }
    }
}

impl Interpolatable for Capsule {
    fn interpolate(&self, other: &Self, progress: f32) -> Self {
        Self {
            left: self.left + (other.left - self.left) * progress,
            top: self.top + (other.top - self.top) * progress,
            width: self.width + (other.width - self.width) * progress,
            height: self.height + (other.height - self.height) * progress,
            color: (self.color.to_vec() + (other.color.to_vec() - self.color.to_vec()) * progress).into(),
        }
    }
}