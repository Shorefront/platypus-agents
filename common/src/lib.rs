use config::Config;
use std::sync::OnceLock;
use log::info;
use serde::Deserialize;

pub fn get<'a, T: serde::Deserialize<'a>>(path: &str) -> Option<T> {
    config().get::<T>(path).ok()
}

fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        Config::builder()
            .add_source(
                config::Environment::with_prefix("PLATYPUS")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .unwrap()
    })
}  

pub fn register(_end_point : &str) {
    // Registration logic here
    // USe reqwest to register end-point with Platypus
    let platypus_server = get::<String>("SERVER").expect("Platypus server URL not configured");
    log::info!("Platypus: {platypus_server}");
}
