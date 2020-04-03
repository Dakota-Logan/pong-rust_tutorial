//!Pong Tutorial 1
use amethyst::{
	prelude::*,
	renderer::{
		plugins::{RenderFlat2D, RenderToWindow},
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
	
	let app_root = application_root_dir()?;
	let display_config_path = app_root.join("config").join("display.ron");
	
	//Creating a new instance of GameDataBuilder, a central repository of all the game logic that runs periodically during the game runtime.
	let game_data = GameDataBuilder::default();
	
	//
	let assets_dir = app_root.join("assets");
	//Combining the builder with the game struct (Pong), creating the overarching Amethyst's root object: Application.
	//It (Application) binds the OS event loop, state machines, timers and other core components in a central place.
	let mut game = Application::new(assets_dir, Pong, game_data)?;
	//Starting the game loop.
	game.run();
	
	Ok(())
}