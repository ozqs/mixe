use macroquad::prelude::*;
use silk_ui::fluent_rounded_rectangle::FluentRoundedRectangle;

#[macroquad::main("Easing Interpolation Movement")]
async fn main() {
    // 初始化一个圆角矩形
    let mut rect = FluentRoundedRectangle::new(100.0, 100.0, 80.0, 60.0, 15.0, GRAY, 1.0);

    loop {
        clear_background(WHITE);

        // 当按下鼠标左键时，设置新的目标位置和大小
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            // 随机设置目标大小作为示例
            let target_width = 100.0 + rand::gen_range(0.0, 100.0);
            let target_height = 100.0 + rand::gen_range(0.0, 100.0);
            rect.set_target(mouse_pos.0, mouse_pos.1, target_width, target_height);
        }

        // 更新矩形位置和大小
        rect.update(get_frame_time());

        // 绘制矩形
        rect.draw();

        next_frame().await;
    }
}
