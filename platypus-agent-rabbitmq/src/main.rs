use log::info;

fn main() {
    let pkg = env!("CARGO_PKG_NAME");
    let ver = env!("CARGO_PKG_VERSION");
    env_logger::init();
    info!("Version\t: {} v{}", pkg, ver);

    // Step 1: Register call back at configured Platypus server URL
    // Step 2: Start listening for messages from Platypus server delivered to call back URL
}
