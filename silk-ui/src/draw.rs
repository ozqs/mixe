use macroquad::prelude::*;
pub fn draw_rounded_rectangle(x: f32, y: f32, width: f32, height: f32, radius: f32, color: Color) {
    // 绘制中心矩形
    draw_rectangle(x + radius, y, width - 2.0 * radius, height, color);
    draw_rectangle(x, y + radius, width, height - 2.0 * radius, color);
    draw_rectangle(x + radius, y + height - radius, width - 2.0 * radius, radius, color);

    // 绘制四个圆角
    draw_circle(x + radius, y + radius, radius, color);
    draw_circle(x + width - radius, y + radius, radius, color);
    draw_circle(x + radius, y + height - radius, radius, color);
    draw_circle(x + width - radius, y + height - radius, radius, color);
}

/// 绘制横向的胶囊体
pub fn draw_horizontal_capsule(x: f32, y: f32, width: f32, radius: f32, color: Color) {
    let height = 2.0 * radius;

    // 确保宽度不小于两倍半径，保证胶囊体合理
    if width < 2.0 * radius {
        println!("宽度太小，无法绘制胶囊体");
        return;
    }

    // 绘制中心矩形
    draw_rectangle(x + radius, y, width - 2.0 * radius, height, color);

    // 绘制左侧半圆
    draw_circle(x + radius, y + radius, radius, color);

    // 绘制右侧半圆
    draw_circle(x + width - radius, y + radius, radius, color);
}

/// 根据左上角坐标绘制文本
pub fn draw_text_top_left(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    // 测量文本尺寸
    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    
    // 计算基线对齐的 y 坐标
    let adjusted_y = y + text_dimensions.offset_y;
    
    // 绘制文本
    draw_text(text, x, adjusted_y, font_size, color);
}

/// 绘制文本
pub fn draw_text_offseted(text: &str, x: f32, y: f32, font_size: f32, offset_y: f32, color: Color) {
    // 计算基线对齐的 y 坐标
    let adjusted_y = y + offset_y;
    
    // 绘制文本
    draw_text(text, x, adjusted_y, font_size, color);
}