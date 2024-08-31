use crate::{
    draw::{draw_horizontal_capsule, draw_text_top_left},
    fluent_shapes::FluentCapsule,
};
use macroquad::prelude::*;

const PADDING: f32 = 5.;

#[allow(unused)]
pub struct SlidingOptionBar {
    start_pos: Vec2,
    hint_text: String,
    options: Vec<String>,
    current_index: usize,
    bar: FluentCapsule,
    target_height: f32,
    hint_rect: Rect,
    option_rects: Vec<Rect>,
    hint_text_size: f32,
    option_text_size: f32,
}

impl SlidingOptionBar {
    pub fn new(
        start_pos: Vec2,
        hint_text: &str,
        options: Vec<&str>,
        initial_index: usize,
        target_height: f32,
        bar_color: Color,
        duration_secs: f32,
    ) -> Self {
        let options = options
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let (hint_rect, option_rects, hint_text_size, option_text_size) =
            Self::measure_options(start_pos, hint_text, &options, target_height);

        let initial_rect = option_rects[initial_index].clone();
        let bar = FluentCapsule::new(
            initial_rect.x - PADDING,
            initial_rect.y - PADDING,
            initial_rect.w + PADDING * 2.,
            initial_rect.h + PADDING * 2.,
            bar_color,
            duration_secs,
        );

        Self {
            start_pos,
            hint_text: hint_text.to_string(),
            options,
            current_index: initial_index,
            bar,
            target_height,
            hint_rect,
            option_rects,
            hint_text_size,
            option_text_size,
        }
    }

    fn measure_options(
        start_pos: Vec2,
        hint_text: &str,
        options: &Vec<String>,
        target_height: f32,
    ) -> (Rect, Vec<Rect>, f32, f32) {
        let mut option_rects = Vec::with_capacity(options.len());

        let hint_text_size = target_height - PADDING * 4.;
        let option_text_size = target_height - PADDING * 4.;
        // let hint_text_size = 20f32;
        // let option_text_size = 20f32;

        // 计算提示文字的坐标
        let hint_size = measure_text(hint_text, None, hint_text_size as u16, 1.0);
        let hint_pos = vec2(
            start_pos.x + PADDING,
            start_pos.y + target_height / 2. - hint_size.height / 2.,
        );

        let mut x = hint_pos.x + PADDING + hint_size.width + PADDING + target_height / 2.;

        // 测量选项文字大小和位置
        for option in options {
            let text_size = measure_text(option, None, option_text_size as u16, 1.0);
            let option_y = start_pos.y + target_height / 2. - text_size.height / 2.;
            let rect = Rect::new(x, option_y, text_size.width, text_size.height);
            option_rects.push(rect);
            x += text_size.width + target_height / 2.;
        }

        (
            Rect::new(hint_pos.x, hint_pos.y, hint_size.width, hint_size.height),
            option_rects,
            hint_text_size,
            option_text_size,
        )
    }

    // 设置当前选项并更新选项条
    pub fn select_option(&mut self, index: usize) {
        if index < self.options.len() {
            self.current_index = index;
            let rect = &self.option_rects[index];
            self.bar.set_target(
                rect.x - PADDING,
                rect.y - PADDING,
                rect.w + PADDING * 2.,
                rect.h + PADDING * 2.,
            ); // 使用 w 和 h
        }
    }

    // 更新选项条状态
    pub fn update(&mut self, delta: f32) {
        self.bar.update(delta);
    }

    // 绘制选项条
    pub fn draw(&self) {
        let big_rect = Rect::new(
            self.option_rects[0].x,
            self.option_rects[0].y,
            self.option_rects[self.option_rects.len() - 1].w
                + (self.option_rects[self.option_rects.len() - 1].x - self.option_rects[0].x),
            self.option_rects[self.option_rects.len() - 1].h,
        );

        draw_horizontal_capsule(
            big_rect.x - 2. * PADDING,
            big_rect.y - 3. * PADDING,
            big_rect.w + 4. * PADDING,
            big_rect.h + 2. * PADDING,
            LIGHTGRAY,
        );

        // 绘制当前选项条（在选项下方）
        self.bar.draw();

        // 绘制提示文字在第一个选项的正左方
        draw_text_top_left(
            &self.hint_text,
            self.hint_rect.x,
            self.hint_rect.y,
            self.hint_text_size,
            BLACK,
        );

        // 绘制选项
        for (i, option) in self.options.iter().enumerate() {
            let rect = &self.option_rects[i];
            draw_text_top_left(option, rect.x, rect.y, self.option_text_size, BLACK);
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
