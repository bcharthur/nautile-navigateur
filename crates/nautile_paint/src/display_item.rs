/// Paint command.
#[derive(Debug, Clone)]
pub enum DisplayItem {
    SolidColor { rgba: [f32; 4] },
    Text { text: String },
}
