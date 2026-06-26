#[derive(Debug, Clone)]
pub struct Color { pub r: u8, pub g: u8, pub b: u8, pub a: u8 }

impl Color {
    pub const TRANSPARENT: Self = Self { r: 0, g: 0, b: 0, a: 0 };
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255, a: 255 };
}

#[derive(Debug, Clone)]
pub struct Rect { pub x: f32, pub y: f32, pub w: f32, pub h: f32 }

#[derive(Debug, Clone)]
pub enum DisplayItem {
    DrawRect { rect: Rect, color: Color },
    DrawBorder { rect: Rect, color: Color, widths: [f32; 4], radii: [f32; 4] },
    DrawText { x: f32, y: f32, text: String, font_size: f32, color: Color },
    DrawImage { rect: Rect, image_id: u32 },
    DrawBoxShadow { rect: Rect, color: Color, blur: f32, spread: f32, offset: (f32, f32) },
    PushClipRect(Rect),
    PopClip,
    PushTransform([f32; 6]),
    PopTransform,
    PushOpacity(f32),
    PopOpacity,
}

#[derive(Debug, Default)]
pub struct DisplayList {
    pub items: Vec<DisplayItem>,
}

impl DisplayList {
    pub fn push(&mut self, item: DisplayItem) { self.items.push(item); }
    pub fn clear(&mut self) { self.items.clear(); }
}
