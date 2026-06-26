use crate::geometry::PhysicalRect;

pub struct Fragment {
    pub rect: PhysicalRect,
    pub baseline: Option<f32>,
    pub children: Vec<Fragment>,
}

impl Fragment {
    pub fn new(rect: PhysicalRect) -> Self {
        Self { rect, baseline: None, children: Vec::new() }
    }
}
