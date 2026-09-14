
use common::register;
use log::info;
use config::{Config,Environment};
use serde::Deserialize;

#[derive(Debug,Deserialize)]
struct Settings {
    pub platypus_host : String,
    pub platypus_port : u16,
    pub platypus_tls : bool,
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

    register("platypus.server",config.platypus_host);

    Ok(())
}
