use core_graphics::event::{
    CGEvent, CGEventTapLocation, CGEventType, CGMouseButton, EventField, ScrollEventUnit,
};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use core_graphics::geometry::CGPoint;
use rand::Rng;

use crate::geometry::PointF64;

// use super::MouseError;

#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    LeftButton = CGMouseButton::Left as isize,
    RightButton = CGMouseButton::Right as isize,
    CenterButton = CGMouseButton::Center as isize,
}

impl From<MouseButton> for CGMouseButton {
    fn from(b: MouseButton) -> Self {
        match b {
            MouseButton::LeftButton => Self::Left,
            MouseButton::RightButton => Self::Right,
            MouseButton::CenterButton => Self::Center,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Toggle {
    Up,
    Down,
}

pub fn move_to(point: PointF64) {
    let point = CGPoint::from(point);
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
    let event = CGEvent::new_mouse_event(
        source,
        CGEventType::MouseMoved,
        CGPoint::from(point),
        CGMouseButton::Left,
    )
    .unwrap();

    calculate_deltas(&event, point.into());

    event.post(CGEventTapLocation::Session);
}

pub fn drag_mouse(point: PointF64, button: MouseButton) {
    let drag_type = mouse_drag_to_CGEventType(button);
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
    let event = CGEvent::new_mouse_event(source, drag_type, point.into(), button.into()).unwrap();

    calculate_deltas(&event, point);

    event.post(CGEventTapLocation::Session);
}

pub fn toggle_mouse(button: MouseButton, tog: Toggle) {
    let point = location_inner();
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
    let mouse_type = mouse_to_CGEventType(button, tog);
    let event = CGEvent::new_mouse_event(source, mouse_type, point, button.into()).unwrap();
    event.post(CGEventTapLocation::Session);
}

pub fn click_mouse(button: MouseButton) {
    toggle_mouse(button, Toggle::Down);
    std::thread::sleep(std::time::Duration::from_micros(100));
    toggle_mouse(button, Toggle::Up);
}

pub fn double_click_mouse(button: MouseButton) {
    let point = location_inner();
    let mouse_type_down = mouse_to_CGEventType(button, Toggle::Down);
    let mouse_type_up = mouse_to_CGEventType(button, Toggle::Up);
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
    // `mouseButton' is ignored unless `mouseType' is one of `kCGEventOtherMouseDown',
    // `kCGEventOtherMouseDragged', or `kCGEventOtherMouseUp'.
    let event = CGEvent::new_mouse_event(source, mouse_type_down, point, button.into()).unwrap();

    event.set_integer_value_field(EventField::MOUSE_EVENT_CLICK_STATE, 2);
    event.post(CGEventTapLocation::HID);

    event.set_type(mouse_type_up);
    event.post(CGEventTapLocation::HID);
}

pub fn scroll_mouse_to(x: i32, y: i32) {
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
    let event = CGEvent::new_scroll_event(source, ScrollEventUnit::PIXEL, 2, y, x, 0).unwrap();
    event.post(CGEventTapLocation::HID);
}

pub fn location() -> PointF64 {
    PointF64::from(location_inner())
}

pub fn smoothly_move_mouse(end_point: PointF64, duration: Option<f64>) {
    let start_point = location();
    let distance = (start_point.x() - end_point.x()).hypot(start_point.y() - end_point.y());
    let step_count = distance.ceil() as i64;
    let interval: u64 = duration
        .map(|d| (d * 1000.0) / distance)
        .unwrap_or(1.0)
        .round() as u64;

    for step in 1..=step_count {
        let position = PointF64::new(
            (end_point.x() - start_point.x()) * (step as f64 / step_count as f64) + start_point.x(),
            (end_point.y() - start_point.y()) * (step as f64 / step_count as f64) + start_point.y(),
        );

        move_to(position);
        std::thread::sleep(std::time::Duration::from_millis(interval));
    }
}

pub fn natually_move_mouse(end_point: PointF64, low_sleep: f64, high_sleep: f64) {
    let mut rng = rand::thread_rng();
    let mut point = location();
    let mut velo_x = 0.0;
    let mut velo_y = 0.0;
    let mut distance;
    while {
        distance = (point.x() - end_point.x()).hypot(point.y() - end_point.y());
        distance
    } > 1.0
    {
        let gravity = rng.gen_range(5.0..500.0);
        velo_x += (gravity * (end_point.x() - point.x())) / distance;
        velo_y += (gravity * (end_point.y() - point.y())) / distance;
        let velo_distance = velo_x.hypot(velo_y);
        velo_x /= velo_distance;
        velo_y /= velo_distance;

        point.add_x(velo_x);
        point.add_y(velo_y);

        move_to(point);

        std::thread::sleep(std::time::Duration::from_millis(
            (rng.gen_range(low_sleep..high_sleep)) as u64,
        ));
    }
}

#[inline(always)]
fn location_inner() -> CGPoint {
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
    let event = CGEvent::new(source).unwrap();
    event.location()
}

#[allow(non_snake_case)]
fn mouse_to_CGEventType(button: MouseButton, tog: Toggle) -> CGEventType {
    match (button, tog) {
        (MouseButton::LeftButton, Toggle::Down) => CGEventType::LeftMouseDown,
        (MouseButton::LeftButton, Toggle::Up) => CGEventType::LeftMouseUp,
        (MouseButton::RightButton, Toggle::Down) => CGEventType::RightMouseDown,
        (MouseButton::RightButton, Toggle::Up) => CGEventType::RightMouseUp,
        (_, Toggle::Down) => CGEventType::OtherMouseDown,
        (_, Toggle::Up) => CGEventType::OtherMouseUp,
    }
}

#[allow(non_snake_case)]
fn mouse_drag_to_CGEventType(button: MouseButton) -> CGEventType {
    match button {
        MouseButton::LeftButton => CGEventType::LeftMouseDragged,
        MouseButton::RightButton => CGEventType::RightMouseDragged,
        MouseButton::CenterButton => CGEventType::OtherMouseDragged,
    }
}

fn calculate_deltas(event: &CGEvent, point: PointF64) {
    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
    let get = CGEvent::new(source).unwrap();
    let mouse = get.location();

    let delta_x = point.x() - mouse.x;
    let delta_y = point.y() - mouse.y;

    // event.set_integer_value_field(EventField::MOUSE_EVENT_DELTA_X, delta_x as i64);
    // event.set_integer_value_field(EventField::MOUSE_EVENT_DELTA_Y, delta_y as i64);

    event.set_double_value_field(EventField::MOUSE_EVENT_DELTA_X, delta_x);
    event.set_double_value_field(EventField::MOUSE_EVENT_DELTA_Y, delta_y);
}
