use log::{debug, error, info, trace, warn};

mod logs;

fn main() {
    logs::setup().expect("unable to configure logger");

    info!("setup!");
}
