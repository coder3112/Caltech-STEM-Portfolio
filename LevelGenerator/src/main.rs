#[macro_use]
extern crate arrayref;

use notan::draw::DrawConfig;
use notan::egui::*;
use notan::log::{debug, LogConfig};
use notan::prelude::*;

mod algorithms;
mod config_gui;
mod errors;
pub(crate) mod level;
pub(crate) mod matrix_gen;
mod render;

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::new()
        .vsync(true)
        .lazy_loop(true)
        .high_dpi(true)
        .resizable(true)
        .fullscreen(true);

    notan::init_with(render::setup)
        .add_config(win)
        .add_config(EguiConfig)
        .add_config(DrawConfig)
        .add_config(LogConfig::debug())
        .draw(render::draw)
        .build()
}

