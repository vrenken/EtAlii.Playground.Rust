use serde::Deserialize;

#[derive(Deserialize)]
pub struct NameForm {
    pub value: String,
}

#[derive(Deserialize)]
pub struct ItemForm {
    pub name: String,
    pub quantity: u32
}
