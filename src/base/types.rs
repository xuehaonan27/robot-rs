use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct PointF64 {
    x: f64,
    y: f64,
}

impl PointF64 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[inline(always)]
    pub fn y(&self) -> f64 {
        self.y
    }

    #[inline(always)]
    pub fn add_x(&mut self, delta_x: f64) {
        self.x += delta_x;
    }

    #[inline(always)]
    pub fn add_y(&mut self, delta_y: f64) {
        self.y += delta_y;
    }
}

impl Display for PointF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl From<PointF64> for core_graphics::display::CGPoint {
    fn from(point: PointF64) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

impl From<core_graphics::display::CGPoint> for PointF64 {
    fn from(point: core_graphics::display::CGPoint) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}
