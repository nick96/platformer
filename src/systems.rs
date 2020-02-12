use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::Player;
use crate::consts::{ARENA_HEIGHT, ARENA_WIDTH, PLAYER_HEIGHT};

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transfs, players, input): Self::SystemData) {
        for (_, transf) in (&players, &mut transfs).join() {
            let scaled_amount = 1.2 as f32;
            let y = transf.translation().y;
            let trans_y = (y + scaled_amount)
                .min(ARENA_HEIGHT - PLAYER_HEIGHT * 0.5)
                .max(PLAYER_HEIGHT * 0.5);
            transf.set_translation_y(trans_y);
        }
    }
}
