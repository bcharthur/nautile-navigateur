use crate::display_list::{Color, DisplayItem, DisplayList, Rect};

pub struct PaintContext<'a> {
    pub list: &'a mut DisplayList,
    pub clip: Option<Rect>,
}

impl<'a> PaintContext<'a> {
    pub fn new(list: &'a mut DisplayList) -> Self { Self { list, clip: None } }

    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        self.list.push(DisplayItem::DrawRect { rect, color });
    }

    pub fn draw_text(&mut self, x: f32, y: f32, text: impl Into<String>, size: f32, color: Color) {
        self.list.push(DisplayItem::DrawText { x, y, text: text.into(), font_size: size, color });
    }

    pub fn push_clip(&mut self, rect: Rect) {
        self.list.push(DisplayItem::PushClipRect(rect));
    }

    pub fn pop_clip(&mut self) {
        self.list.push(DisplayItem::PopClip);
    }
}
