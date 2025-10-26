use crate::portal::*;
use askama::Template;

#[derive(Template)]
#[template(path = "items//item.html")]
pub struct ItemTemplate<'a> {
    pub item: &'a ListItem,
}

#[derive(Template)]
#[template(path = "items//page.html")]
pub struct ItemListTemplate<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub items: &'a [ListItem],
}