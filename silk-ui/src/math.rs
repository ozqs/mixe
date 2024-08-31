// 缓动函数
pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powf(3.0) / 2.0
    }
}

pub struct Fluent<T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<f32, Output = T>
        + Copy
        + PartialOrd
        + std::fmt::Debug,
{
    pub value: T,
    pub target_value: T,
    pub progress: f32,
    pub duration_secs: f32,
}

impl<T> Fluent<T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<f32, Output = T>
        + Copy
        + PartialOrd
        + std::fmt::Debug,
{
    pub fn new(value: T, duration_secs: f32) -> Self {
        Self {
            value,
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
            self.value = self.value + (self.target_value - self.value) * eased_progress;
        }
        if self.progress >= 1.0 {
            self.progress = 1.0; // 防止进度超过1
        }
    }
}
