use macroquad::prelude::*;
use silk_ui::fluent_rounded_rectangle::FluentRoundedRectangle;
// 定义滑动选项条结构体
pub struct SlidingOptionBar {
    options: Vec<String>,
    current_index: usize,
    hint_text: String,
    option_rects: Vec<Rect>,
    bar: FluentRoundedRectangle,
    padding: f32,
    text_size: f32,
    bar_color: Color,
    option_color: Color,
    hint_x_offset: f32,
    hint_y_offset: f32,
}

impl SlidingOptionBar {
    pub fn new(
        hint_text: &str,
        options: Vec<&str>,
        initial_index: usize,
        bar_color: Color,
        option_color: Color,
        padding: f32,
        text_size: f32,
        bar_radius: f32,
        duration_secs: f32,
    ) -> Self {
        let options = options.into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let mut option_rects = Vec::with_capacity(options.len());
        
        // 计算提示文字的y坐标
        let hint_size = measure_text(hint_text, None, text_size as u16, 1.0);
        let hint_y_offset = hint_size.height / 2.0;
        
        // 计算第一个选项的x坐标
        let first_option_x = hint_size.width + padding;
        
        // 计算提示文字的x坐标
        let hint_x_offset = first_option_x - hint_size.width - padding;
        let mut x = hint_x_offset + padding;

        // 测量选项文字大小和位置
        for option in &options {
            let text_size = measure_text(option, None, text_size as u16, 1.0);
            let rect = Rect::new(x, hint_y_offset,
                text_size.width + padding * 2.0, text_size.height + padding * 2.0);
            option_rects.push(rect);
            x += text_size.width + padding * 2.0;
        }

        let initial_rect = option_rects[initial_index].clone();
        let mut bar = FluentRoundedRectangle::new(
            initial_rect.x,
            initial_rect.y,
            initial_rect.w, // 使用 w 代替 width
            initial_rect.h, // 使用 h 代替 height
            bar_radius,
            bar_color,
            duration_secs
        );

        Self {
            options,
            current_index: initial_index,
            hint_text: hint_text.to_string(),
            option_rects,
            bar,
            padding,
            text_size,
            bar_color,
            option_color,
            hint_x_offset,
            hint_y_offset,
        }
    }

    // 设置当前选项并更新选项条
    pub fn select_option(&mut self, index: usize) {
        if index < self.options.len() {
            self.current_index = index;
            let rect = &self.option_rects[index];
            self.bar.set_target(rect.x, rect.y, rect.w, rect.h); // 使用 w 和 h
        }
    }

    // 更新选项条状态
    pub fn update(&mut self, delta: f32) {
        self.bar.update(delta);
    }

    // 绘制选项条
    pub fn draw(&self) {
        // 绘制当前选项条（在选项下方）
        self.bar.draw();

        // 绘制提示文字在第一个选项的正左方
        draw_text(&self.hint_text, self.hint_x_offset, self.hint_y_offset + 30.0, self.text_size, WHITE);

        // 绘制选项
        for (i, option) in self.options.iter().enumerate() {
            let rect = &self.option_rects[i];
            draw_text(option, rect.x + self.padding, rect.y + self.padding + self.text_size / 2.0, self.text_size, WHITE);
        }
    }

    // 检查点击事件
    pub fn handle_click(&mut self, mouse_pos: Vec2) {
        for (i, rect) in self.option_rects.iter().enumerate() {
            if rect.contains(mouse_pos) {
                self.select_option(i);
                break;
            }
        }
    }
}

// 假设 `FluentRoundedRectangle` 和 `SlidingOptionBar` 已经定义在相应的模块中

#[macroquad::main("Sliding Option Bar Example")]
async fn main() {
    // 定义颜色
    let bar_color = Color::from_rgba(100, 150, 200, 255);
    let option_color = Color::from_rgba(200, 200, 200, 255);

    // 创建 `SlidingOptionBar` 实例
    let options = vec!["Option 1", "Option 2", "Option 3"];
    let initial_index = 0;
    let padding = 10.0;
    let text_size = 20.0;
    let bar_radius = 10.0;
    let duration_secs = 0.5;
    
    let mut option_bar = SlidingOptionBar::new(
        "Choose an option:",
        options,
        initial_index,
        bar_color,
        option_color,
        padding,
        text_size,
        bar_radius,
        duration_secs
    );

    loop {
        clear_background(BLACK);

        // 处理鼠标点击
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            option_bar.handle_click(vec2(mouse_pos.0, mouse_pos.1));
        }

        // 更新选项条状态
        let delta = get_frame_time();
        option_bar.update(delta);

        // 绘制选项条
        option_bar.draw();

        // 刷新屏幕
        next_frame().await;
    }
}
