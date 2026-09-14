use config::Config;
use std::sync::OnceLock;
use log::{debug};
// use serde::Deserialize;

pub fn get<'a, T: serde::Deserialize<'a>>(path: &str) -> Option<T> {
    debug!("Config::get({path})");
    config().get::<T>(path).ok()
}

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        Config::builder()
            .add_source(
                config::Environment::with_prefix("platypus")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .unwrap()
    })
}  

pub fn register(_end_point : &str, platypus : String) {
    // Registration logic here
    // USe reqwest to register end-point with Platypus


    log::info!("Host: {platypus}");
}

/// Generate Platypus URL from components
pub fn get_url(tls : bool, host : String, port : u16) -> String {
    let http = match tls {
        true => "https",
        false => "http"
    };
    format!("{http}://{host}:{port}")
}
