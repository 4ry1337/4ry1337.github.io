use ary1337::{
    config::get_config,
    error::Error,
    routes::{Routes, about::about, home::home},
    tracing::{get_subscriber, init_subscriber},
};
use tracing::info;

fn main() -> Result<(), Error> {
    let subscriber = get_subscriber("ary1337".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    info!("Starting ary1337");
    let config = get_config().expect("Failed to read configuration.");
    let routes = Routes::new(config)
        .route("/home", home())
        .route("/about", about());

    routes.build()?;
    info!("Finished");
    Ok(())
}
