use std::ops::Deref;
use std::ops::Range;

use crate::gfx::Point2;
use crate::gfx::Rect;

use crate::util;

#[derive(Debug, Clone)]
pub enum FrameRange {
    Full,
    Partial(Range<usize>),
}

impl Default for FrameRange {
    fn default() -> Self {
        Self::Full
    }
}

/// Layer identifier.
pub type LayerId = usize;

#[derive(Debug, Default, Clone)]
pub struct Layer {
    /// Frame range.
    pub frames: FrameRange,
    /// Visbility.
    pub is_visible: bool,
    /// Sort order.
    pub index: usize,
}

impl Layer {
    pub fn new(frames: FrameRange, index: usize) -> Self {
        Self {
            frames,
            is_visible: true,
            index,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayerCoords<T>(Point2<T>);

impl<T> LayerCoords<T> {
    pub fn new(x: T, y: T) -> Self {
        Self(Point2::new(x, y))
    }
}

impl LayerCoords<i32> {
    pub fn clamp(&mut self, rect: Rect<i32>) {
        util::clamp(&mut self.0, rect);
    }
}

impl<T> Deref for LayerCoords<T> {
    type Target = Point2<T>;

    fn deref(&self) -> &Point2<T> {
        &self.0
    }
}

impl From<LayerCoords<f32>> for LayerCoords<i32> {
    fn from(other: LayerCoords<f32>) -> LayerCoords<i32> {
        LayerCoords::new(other.x.round() as i32, other.y.round() as i32)
    }
}

impl From<LayerCoords<i32>> for LayerCoords<f32> {
    fn from(other: LayerCoords<i32>) -> LayerCoords<f32> {
        LayerCoords::new(other.x as f32, other.y as f32)
    }
}

impl From<LayerCoords<f32>> for LayerCoords<u32> {
    fn from(other: LayerCoords<f32>) -> LayerCoords<u32> {
        LayerCoords::new(other.x.round() as u32, other.y.round() as u32)
    }
}

impl From<Point2<f32>> for LayerCoords<f32> {
    fn from(p: Point2<f32>) -> LayerCoords<f32> {
        LayerCoords::new(p.x, p.y)
    }
}
