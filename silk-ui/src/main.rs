use macroquad::prelude::*;
use silk_ui::sliding_option_bar::SlidingOptionBar;

#[macroquad::main("Sliding Option Bar Example")]
async fn main() {
    let mut option_bar = SlidingOptionBar::new(
        vec2(100., 100.),
        "Comparison Indicator",
        vec!["SMALL", "EQUAL", "GRATER"],
        0,
        40.,
        GRAY,
        1.0,
    );

    loop {
        clear_background(WHITE);

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
