mod components;
#[rustfmt::skip]
mod config;
mod localize;
mod wayland;
mod wayland_source;
mod wayland_subscription;

use config::APP_ID;
use log::info;

use localize::localize;

use crate::{
    components::app,
    config::{PROFILE, VERSION},
};

fn main() -> iced::Result {
    // Initialize logger
    pretty_env_logger::init();
    info!("Iced Workspaces Applet ({})", APP_ID);
    info!("Version: {} ({})", VERSION, PROFILE);

    // Prepare i18n
    localize();

    app::run()
}
