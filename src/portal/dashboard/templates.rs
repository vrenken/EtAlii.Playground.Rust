use askama::Template;

#[derive(Template)]
#[template(path = "dashboard//page.html")]
pub struct PageTemplate<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
}

#[derive(Template)]
#[template(path = "dashboard//cpu_usage.html")]
pub struct CpuUsageTemplate<'a> {
    pub value: &'a str,
}

#[derive(Template)]
#[template(path = "dashboard//ram_usage.html")]
pub struct RamUsageTemplate<'a> {
    pub value: &'a str
}