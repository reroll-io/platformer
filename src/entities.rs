use amethyst::prelude::{Builder, GameData, SimpleState, StateData, World};
use amethyst::renderer::{Camera, Projection};

pub struct InitialState;

impl SimpleState for InitialState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {}
}
