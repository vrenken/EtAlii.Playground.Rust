use crate::data::*;

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub items: Arc<Mutex<Vec<ListItem>>>,
    pub name: Arc<Mutex<String>>,
}