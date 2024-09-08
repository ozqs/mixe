use macroquad::prelude::*;
use crate::fluent::Fluent;
use crate::draw::*;

pub struct FluentShape {
    position: Fluent<Vec2>,
    size: Fluent<Vec2>,
    color: Color,
}

impl FluentShape {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color, duration_secs: f32) -> Self {
        Self {
            position: Fluent::new(vec2(x, y), duration_secs),
            size: Fluent::new(vec2(width, height), duration_secs),
            color,
        }
    }

    pub fn set_target(&mut self, target_x: f32, target_y: f32, target_width: f32, target_height: f32) {
        self.position.set_target(vec2(target_x, target_y));
        self.size.set_target(vec2(target_width, target_height));
    }

    pub fn update(&mut self, delta: f32) {
        self.position.update(delta);
        self.size.update(delta);
    }

    pub fn is_finished(&self) -> bool {
        self.position.is_finished() && self.size.is_finished()
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

pub struct FluentRoundedRectangle {
    shape: FluentShape,
    radius: Fluent<f32>,
}

impl FluentRoundedRectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, radius: f32, color: Color, duration_secs: f32) -> Self {
        Self {
            shape: FluentShape::new(x, y, width, height, color, duration_secs),
            radius: Fluent::new(radius, duration_secs),
        }
    }

    pub fn set_target(&mut self, target_x: f32, target_y: f32, target_width: f32, target_height: f32, target_radius: f32) {
        self.shape.set_target(target_x, target_y, target_width, target_height);
        self.radius.set_target(target_radius);
    }

    pub fn update(&mut self, delta: f32) {
        self.shape.update(delta);
        self.radius.update(delta);
    }

    pub fn draw(&self) {
        draw_rounded_rectangle(
            self.shape.position.value.x,
            self.shape.position.value.y,
            self.shape.size.value.x,
            self.shape.size.value.y,
            self.radius.value,
            self.shape.color,
        );
    }

    pub fn is_finished(&self) -> bool {
        self.shape.is_finished() && self.radius.is_finished()
    }

    pub fn set_color(&mut self, color: Color) {
        self.shape.set_color(color);
    }
}

pub struct FluentCapsule {
    shape: FluentShape,
}

impl FluentCapsule {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color, duration_secs: f32) -> Self {
        Self {
            shape: FluentShape::new(x, y, width, height, color, duration_secs),
        }
    }

    pub fn set_target(&mut self, target_x: f32, target_y: f32, target_width: f32, target_height: f32) {
        self.shape.set_target(target_x, target_y, target_width, target_height);
    }

    pub fn update(&mut self, delta: f32) {
        self.shape.update(delta);
    }

    pub fn draw(&self) {
        draw_capsule(
            self.shape.position.value.x,
            self.shape.position.value.y,
            self.shape.size.value.x,
            self.shape.size.value.y,
            self.shape.color,
        );
    }

    pub fn is_finished(&self) -> bool {
        self.shape.is_finished()
    }

    pub fn set_color(&mut self, color: Color) {
        self.shape.set_color(color);
    }
}