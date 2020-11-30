use amethyst::{
	prelude::*,
	SimpleState,
};


pub struct GameState;

impl SimpleState for GameState {
	fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
		println!("Hello world");
	}
}
