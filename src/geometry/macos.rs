use core_graphics::display::{CGPoint, CGRect, CGSize};

use super::{PointF64, RectF64, SizeF64};

impl From<PointF64> for core_graphics::display::CGPoint {
    fn from(point: PointF64) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

impl From<CGPoint> for PointF64 {
    fn from(point: core_graphics::display::CGPoint) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

impl From<SizeF64> for CGSize {
    fn from(value: SizeF64) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

impl From<CGSize> for SizeF64 {
    fn from(value: CGSize) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

impl From<CGRect> for RectF64 {
    fn from(value: CGRect) -> Self {
        Self {
            origin: value.origin.into(),
            size: value.size.into(),
        }
    }
}

impl From<RectF64> for CGRect {
    fn from(value: RectF64) -> Self {
        Self {
            origin: value.origin.into(),
            size: value.size.into(),
        }
    }
}
