use std::sync::{Arc, Mutex};
use serde::Deserialize;

#[derive(Clone)]
pub struct AppState {
    pub items: Arc<Mutex<Vec<ListItem>>>,
    pub name: Arc<Mutex<String>>,
}



#[derive(Deserialize)]
pub struct NameForm {
    pub value: String,
}


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
