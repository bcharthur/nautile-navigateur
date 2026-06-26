#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect<T> {
    pub origin: Point<T>,
    pub size: Size<T>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edges<T> {
    pub top: T,
    pub right: T,
    pub bottom: T,
    pub left: T,
}

pub type PhysicalPoint = Point<f32>;
pub type PhysicalSize = Size<f32>;
pub type PhysicalRect = Rect<f32>;

impl<T: Copy + std::ops::Add<Output = T>> Rect<T> {
    pub fn max_x(&self) -> T { self.origin.x + self.size.width }
    pub fn max_y(&self) -> T { self.origin.y + self.size.height }
}
