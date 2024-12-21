use std::fmt::Display;

pub mod macos;

#[derive(Debug, Clone, Copy)]
pub struct PointF64 {
    x: f64,
    y: f64,
}

impl PointF64 {
    pub const ZERO: PointF64 = PointF64 { x: 0.0, y: 0.0 };

    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn x(&self) -> f64 {
        self.x
    }

    #[inline]
    pub const fn y(&self) -> f64 {
        self.y
    }

    #[inline]
    pub fn add_x(&mut self, delta_x: f64) {
        self.x += delta_x;
    }

    #[inline]
    pub fn add_y(&mut self, delta_y: f64) {
        self.y += delta_y;
    }
}

impl Display for PointF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SizeF64 {
    width: f64,
    height: f64,
}

impl SizeF64 {
    pub const ZERO: SizeF64 = SizeF64 {
        width: 0.0,
        height: 0.0,
    };

    #[inline]
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    #[inline]
    pub const fn width(&self) -> f64 {
        self.width
    }

    #[inline]
    pub const fn height(&self) -> f64 {
        self.height
    }
}

impl Display for SizeF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RectF64 {
    origin: PointF64,
    size: SizeF64,
}

impl RectF64 {
    pub const ZERO: RectF64 = RectF64 {
        origin: PointF64::ZERO,
        size: SizeF64::ZERO,
    };

    #[inline]
    pub fn new(origin: PointF64, size: SizeF64) -> Self {
        Self { origin, size }
    }

    #[inline]
    pub fn origin(&self) -> PointF64 {
        self.origin
    }

    #[inline]
    pub const fn size(&self) -> SizeF64 {
        self.size
    }

    #[inline]
    pub fn max_x(&self) -> f64 {
        self.origin.x() + self.size().width()
    }

    #[inline]
    pub fn max_y(&self) -> f64 {
        self.origin.y() + self.size().height()
    }

    #[inline]
    pub fn is_point_visible(&self, point: PointF64) -> bool {
        point.x() >= self.origin.x()
            && point.y() >= self.origin.y()
            && point.x() < self.max_x()
            && point.y() < self.max_y()
    }

    #[inline]
    pub fn is_rect_visible(&self, rect: RectF64) -> bool {
        self.is_point_visible(rect.origin)
            && (rect.size().width() + rect.origin().x() + self.origin().x()) <= self.size().width()
            && (rect.size().height() + rect.origin().y() + self.origin().y())
                <= self.size().height()
    }
}

impl Display for RectF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.origin, self.size)
    }
}
