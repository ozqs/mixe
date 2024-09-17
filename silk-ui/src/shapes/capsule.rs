use macroquad::prelude::*;

pub struct Capsule {
    left: f32,
    top: f32,
    width: f32,
    height: f32,
}

impl Capsule {
    fn new(left: f32, top: f32, width: f32, height: f32) -> Self {
        Self {
            left,
            top,
            width,
            height,
        }
    }

    fn center_in(&mut self, left: f32, top: f32) {
        self.left = left - self.width / 2.;
        self.top = top - self.height / 2.;
    }
}

impl From<Rect> for Capsule {
    fn from(value: Rect) -> Self {
        Self {
            left: value.x,
            top: value.y,
            width: value.w,
            height: value.h,
        }
    }
}