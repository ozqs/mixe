use macroquad::prelude::*;
use crate::math::ease_in_out_cubic;
use crate::draw::*;

// 定义圆角矩形结构体
pub struct FluentRoundedRectangle {
    position: Vec2,
    target_position: Vec2,
    width: f32,
    target_width: f32,
    height: f32,
    target_height: f32,
    radius: f32,
    color: Color,
    progress: f32,
    duration_secs: f32,
}

impl FluentRoundedRectangle {
    // 构造函数
    pub fn new(x: f32, y: f32, width: f32, height: f32, radius: f32, color: Color, duration_secs: f32) -> Self {
        Self {
            position: vec2(x, y),
            target_position: vec2(x, y),
            width,
            target_width: width,
            height,
            target_height: height,
            radius,
            color,
            progress: 1.0,
            duration_secs,
        }
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        let rect = Rect::new(self.position.x, self.position.y, self.width, self.height);
        rect.contains(vec2(x, y))
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    // 更新目标位置和大小
    pub fn set_target(&mut self, target_x: f32, target_y: f32, target_width: f32, target_height: f32) {
        self.target_position = vec2(target_x, target_y);
        self.target_width = target_width;
        self.target_height = target_height;
        self.progress = 0.0; // 重置进度
    }

    // 更新位置和大小，使用缓动插值进行平滑过渡
    pub fn update(&mut self, delta: f32) {
        if self.progress < 1.0 {
            self.progress += delta / self.duration_secs; // 控制平滑速度
            let eased_progress = ease_in_out_cubic(self.progress.min(1.0));
            self.position.x = self.position.x + (self.target_position.x - self.position.x) * eased_progress;
            self.position.y = self.position.y + (self.target_position.y - self.position.y) * eased_progress;
            self.width = self.width + (self.target_width - self.width) * eased_progress;
            self.height = self.height + (self.target_height - self.height) * eased_progress;
        }
        if self.progress >= 1.0 {
            self.progress = 1.0; // 防止进度超过1
        }
    }

    pub fn is_finished(&self) -> bool {
        self.progress >= 1.0
    }

    pub fn set_duration(&mut self, duration_secs: f32) {
        self.duration_secs = duration_secs;
    }

    // 绘制矩形，调用独立的绘制函数
    pub fn draw(&self) {
        draw_rounded_rectangle(self.position.x, self.position.y, self.width, self.height, self.radius, self.color);
    }
}

pub struct FluentCapsule {
    position: Vec2,
    target_position: Vec2,
    width: f32,
    target_width: f32,
    height: f32,
    target_height: f32,
    color: Color,
    progress: f32,
    duration_secs: f32,
}

impl FluentCapsule {
    // 构造函数
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color, duration_secs: f32) -> Self {
        Self {
            position: vec2(x, y),
            target_position: vec2(x, y),
            width,
            target_width: width,
            height,
            target_height: height,
            color,
            progress: 1.0,
            duration_secs,
        }
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        let rect = Rect::new(self.position.x, self.position.y, self.width, self.height);
        rect.contains(vec2(x, y))
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    // 更新目标位置和大小
    pub fn set_target(&mut self, target_x: f32, target_y: f32, target_width: f32, target_height: f32) {
        self.target_position = vec2(target_x, target_y);
        self.target_width = target_width;
        self.target_height = target_height;
        self.progress = 0.0; // 重置进度
    }

    // 更新位置和大小，使用缓动插值进行平滑过渡
    pub fn update(&mut self, delta: f32) {
        if self.progress < 1.0 {
            self.progress += delta / self.duration_secs; // 控制平滑速度
            let eased_progress = ease_in_out_cubic(self.progress.min(1.0));
            self.position.x = self.position.x + (self.target_position.x - self.position.x) * eased_progress;
            self.position.y = self.position.y + (self.target_position.y - self.position.y) * eased_progress;
            self.width = self.width + (self.target_width - self.width) * eased_progress;
            self.height = self.height + (self.target_height - self.height) * eased_progress;
        }
        if self.progress >= 1.0 {
            self.progress = 1.0; // 防止进度超过1
        }
    }

    pub fn is_finished(&self) -> bool {
        self.progress >= 1.0
    }

    pub fn set_duration(&mut self, duration_secs: f32) {
        self.duration_secs = duration_secs;
    }

    // 绘制矩形，调用独立的绘制函数
    pub fn draw(&self) {
        draw_horizontal_capsule(self.position.x, self.position.y, self.width, self.height / 2., self.color);
    }
}