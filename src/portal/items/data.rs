use serde::Deserialize;

#[derive(Clone)]
pub struct ListItem {
    pub name: String,
    pub quantity: u32,
}

impl ListItem {
    pub fn new(name: &str, quantity: u32) -> Self {
        Self {
            name: name.to_string(),
            quantity,
        }
    }
}

#[derive(Deserialize)]
pub struct ItemForm {
    pub name: String,
    pub quantity: u32
}
