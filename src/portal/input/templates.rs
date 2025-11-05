use askama::Template;

#[derive(Template)]
#[template(path = "input//page.html")]
pub struct PageTemplate<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub value: &'a str,
    pub is_authenticated: bool,
}
