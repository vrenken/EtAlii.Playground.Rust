use crate::data::ListItem;
use askama::Template;

#[derive(Template)]
#[template(path = "input.html")]
pub struct InputTemplate<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub value: &'a str,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub value: &'a str,
}

#[derive(Template)]
#[template(path = "item.html")]
pub struct ItemTemplate<'a> {
    pub item: &'a ListItem,
}

#[derive(Template)]
#[template(path = "item_list.html")]
pub struct ItemListTemplate<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub items: &'a [ListItem],
}