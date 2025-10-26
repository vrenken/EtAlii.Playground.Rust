use askama::Template;

#[derive(Template)]
#[template(path = "dashboard//page.html")]
pub struct PageTemplate<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub value: &'a str,
}