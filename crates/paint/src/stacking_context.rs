use crate::display_list::DisplayList;

pub struct StackingContext {
    pub z_index: i32,
    pub opacity: f32,
    pub transform: Option<[f32; 6]>,
    pub display_list: DisplayList,
    pub children: Vec<StackingContext>,
}

impl StackingContext {
    pub fn root() -> Self {
        Self { z_index: 0, opacity: 1.0, transform: None, display_list: DisplayList::default(), children: Vec::new() }
    }

    pub fn flatten_sorted(&self) -> Vec<&crate::display_list::DisplayItem> {
        let mut result: Vec<_> = self.display_list.items.iter().collect();
        let mut sorted_children: Vec<_> = self.children.iter().collect();
        sorted_children.sort_by_key(|c| c.z_index);
        for child in sorted_children {
            result.extend(child.flatten_sorted());
        }
        result
    }
}
