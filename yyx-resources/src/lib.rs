#![warn(clippy::all)]

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "yyx-resources/assets"]
pub struct YyxAsset;
