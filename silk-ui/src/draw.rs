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

/// 绘制胶囊体
/// 
/// 参数:
/// - x: 胶囊体左上角或左端点的x坐标
/// - y: 胶囊体左上角或上端点的y坐标
/// - width: 水平方向上的尺寸
/// - height: 垂直方向上的尺寸
/// - color: 胶囊体的颜色
pub fn draw_capsule(x: f32, y: f32, width: f32, height: f32, color: Color) {
    let radius = 0.5 * if width > height { height } else { width };

    if width > height {
        // 横向胶囊体
        draw_rectangle(x + radius, y, width - 2.0 * radius, height, color);
        draw_circle(x + radius, y + radius, radius, color);
        draw_circle(x + width - radius, y + radius, radius, color);
    } else {
        // 纵向胶囊体
        draw_rectangle(x, y + radius, width, height - 2.0 * radius, color);
        draw_circle(x + radius, y + radius, radius, color);
        draw_circle(x + radius, y + height - radius, radius, color);
    }
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