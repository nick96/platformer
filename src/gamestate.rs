use amethyst::prelude::*;

use crate::assets::load_sprite_sheet;
use crate::components::{init_camera, init_player, Player};

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world, "textures/skeleton_spritesheet");
        world.register::<Player>();
        init_player(world, sprite_sheet_handle.clone());
        init_camera(world);
    }
}
