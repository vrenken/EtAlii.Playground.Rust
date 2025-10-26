use askama::Template;

#[derive(Template)]
#[template(path = "input//page.html")]
pub struct InputTemplate<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub value: &'a str,
}
