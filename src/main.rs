//!Pong Tutorial 1
use amethyst::{
	prelude::*,
	rederer::{
		plugins::{RenderFlat2d, RenderTowindow},
		types::DefaultBackend,
		RenderingBundle,
	},
	utils::application_root_dir,
};

pub struct Pong;

impl SimpleState for Pong {}

fn main() -> amethyst::Result<()> {
	//logs errors, warnings, and debug messages
	amethyst::start_logger(Default::default());
	
	Ok(())
}