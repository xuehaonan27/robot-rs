use std::{thread::sleep, time::Duration};

use robot_rs::{
    geometry::PointF64,
    mouse::macos::{
        click_mouse, double_click_mouse, drag_mouse, location, move_to, natually_move_mouse,
        scroll_mouse_to, smoothly_move_mouse, MouseButton,
    },
    screen::macos::{main_display_rect, main_display_scale},
};

fn main() {
    // for _ in 0..100 {
    //     println!("{}", location());
    //     sleep(Duration::from_secs(1));
    // }

    // move_to(PointF64::new(590.0, 365.0));
    // sleep(Duration::from_secs(1));
    // click_mouse(MouseButton::LeftButton);
    // sleep(Duration::from_secs(1));
    // double_click_mouse(MouseButton::LeftButton);
    // sleep(Duration::from_secs(1));
    // drag_mouse(PointF64::new(670.0, 300.0), MouseButton::LeftButton);

    // let point = location();
    // natually_move_mouse(PointF64::new(590.0, 365.0), 1.0, 3.0);
    // smoothly_move_mouse(point, Some(5.0));

    // smoothly_move_mouse(PointF64::new(590.0, 365.0), Some(2.0));
    // scroll_mouse_to(100, 1000);

    println!("rect = {}", main_display_rect());
    println!("scale = {}", main_display_scale());
}
