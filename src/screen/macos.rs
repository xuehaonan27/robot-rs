use core_graphics::display::CGDisplay;

use crate::geometry::{RectF64, SizeF64};

pub fn main_display_scale() -> f64 {
    scale(None)
}

pub fn scale(display_id: Option<u32>) -> f64 {
    let display = if let Some(display_id) = display_id {
        CGDisplay::new(display_id)
    } else {
        CGDisplay::main()
    };
    let mode = display.display_mode().unwrap();
    let pixel_width = mode.pixel_width();
    let target_width = mode.width();
    (pixel_width as f64) / (target_width as f64)
}

pub fn main_display_rect() -> RectF64 {
    rect(None)
}

pub fn rect(display_id: Option<u32>) -> RectF64 {
    let display = if let Some(display_id) = display_id {
        CGDisplay::new(display_id)
    } else {
        CGDisplay::main()
    };

    let display_rect = display.bounds();
    display_rect.into()
}

pub fn main_display_size() -> SizeF64 {
    size(None)
}

pub fn size(display_id: Option<u32>) -> SizeF64 {
    let rect = rect(display_id);
    rect.size()
}
