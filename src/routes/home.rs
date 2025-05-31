use askama::Template;

#[derive(Template)]
#[template(path = "home/index.html")]
struct HomeTemplate<'a> {
    title: &'a str,
    description: &'a str,
}

pub fn home() -> impl Template {
    HomeTemplate {
        title: "Rakhat Yskak",
        description: "Rust App",
    }
}
