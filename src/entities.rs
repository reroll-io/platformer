use amethyst::core::Transform;
use amethyst::prelude::{Builder, GameData, SimpleState, StateData, World};
use amethyst::renderer::{Camera, Projection};

pub const DISPLAY_WIDTH: f32 = 512.;
pub const DISPLAY_HEIGHT: f32 = 320.;

pub struct InitialState;

impl SimpleState for InitialState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {}
}
