use askama::Template;

#[derive(Template)]
#[template(path = "pages/about.html")]
struct AboutTemplate<'a> {
    title: &'a str,
    description: &'a str,
}

pub fn about() -> impl Template {
    AboutTemplate {
        title: "Rakhat Yskak",
        description: "Rust App",
    }
}
