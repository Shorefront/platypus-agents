
use common::{get_url,register};
use log::info;
use config::{Config,Environment};
use serde::Deserialize;

#[derive(Debug,Deserialize)]
struct Settings {
    pub host : String,
    pub port : u16,
    pub tls : Option<bool>,
}

fn main() -> Result<(),config::ConfigError> {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");
    env_logger::init();
    info!("Version\t: {} v{}", pkg, ver);

    let settings = Config::builder()
        .set_default("port",8001)?
        .set_default("tls",true)?
        .add_source(
            Environment::with_prefix("PLATYPUS")
                .try_parsing(true)
                .separator("_"),
        )
        .build()?;

    let config : Settings = settings.try_deserialize()?;

    let url = get_url(config.tls.unwrap_or_default(),config.host,config.port);

    register("platypus.server",url);

    Ok(())
}
