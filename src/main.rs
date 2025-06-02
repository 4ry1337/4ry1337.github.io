use ary1337::{
    config::get_config,
    error::Error,
    routes::{Routes, about::about, home::home},
};

fn main() -> Result<(), Error> {
    let config = get_config().expect("Failed to read configuration.");

    let routes = Routes::new(config)
        .route("/", home())
        .route("/about", about());

    routes.build()?;
    Ok(())
}
