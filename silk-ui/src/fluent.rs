use macroquad::prelude::*;
use crate::math::ease_in_out_cubic;

pub mod fluent_shapes;

pub trait Interpolatable {
    fn interpolate(&self, other: &Self, progress: f32) -> Self;
}

impl<T> Interpolatable for T
where
    T: std::ops::Add<Output=T>
    + std::ops::Sub<Output=T>
    + std::ops::Mul<f32, Output=T>
    + Clone,
{
    fn interpolate(&self, other: &Self, progress: f32) -> Self {
        (other.clone() - self.clone()) * progress + self.clone()
    }
}

pub struct Fluent<T>
where
    T: Interpolatable + Clone,
{
    pub value: T,
    pub target_value: T,
    pub progress: f32,
    pub duration_secs: f32,
}

impl<T> Fluent<T>
where
    T: Interpolatable + Clone,
{
    pub fn new(value: T, duration_secs: f32) -> Self {
        Self {
            value: value.clone(),
            target_value: value,
            progress: 1.0,
            duration_secs,
        }
    }

    pub fn set_target(&mut self, target_value: T) {
        self.target_value = target_value;
        self.progress = 0.0; // 重置进度
    }

    pub fn update(&mut self, delta: f32) {
        if self.progress < 1.0 {
            self.progress += delta / self.duration_secs; // 控制平滑速度
            let eased_progress = ease_in_out_cubic(self.progress.min(1.0));
            // self.value = self.value.clone() + (self.target_value.clone() - self.value.clone()) * eased_progress;
            self.value = self.value.interpolate(&self.target_value, eased_progress);
        }
        if self.progress >= 1.0 {
            self.progress = 1.0; // 防止进度超过1
        }
    }

    pub fn is_finished(&self) -> bool {
        self.progress >= 1.0
    }
}
