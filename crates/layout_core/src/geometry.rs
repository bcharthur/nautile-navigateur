pub type Length = f32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogicalSize { pub inline: Length, pub block: Length }

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicalRect { pub x: f32, pub y: f32, pub w: f32, pub h: f32 }

impl PhysicalRect {
    pub fn zero() -> Self { Self { x: 0.0, y: 0.0, w: 0.0, h: 0.0 } }
    pub fn max_x(&self) -> f32 { self.x + self.w }
    pub fn max_y(&self) -> f32 { self.y + self.h }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edges<T> { pub top: T, pub right: T, pub bottom: T, pub left: T }

impl<T: Default> Default for Edges<T> {
    fn default() -> Self { Self { top: T::default(), right: T::default(), bottom: T::default(), left: T::default() } }
}

impl Edges<f32> {
    pub fn horizontal(&self) -> f32 { self.left + self.right }
    pub fn vertical(&self) -> f32 { self.top + self.bottom }
}
