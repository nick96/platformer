use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::Player;
use crate::consts::{ARENA_HEIGHT, ARENA_WIDTH, PLAYER_HEIGHT, PLAYER_WIDTH};

/// System to control player movement.
#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    /// Run the player system.
    ///
    /// Iterates over the players (1) and updates their position based on the
    /// input data. The player cannot be moved out of the bounds of
    /// `ARENA_WIDTH` and `ARENA_HEIGHT`, if they try to the player won't move
    /// any further.
    ///
    /// See bindings_config.ron for the actual bindings.
    fn run(&mut self, (mut transfs, players, input): Self::SystemData) {
        for (_, transf) in (&players, &mut transfs).join() {
            let min_x = ARENA_WIDTH - PLAYER_WIDTH * 0.5;
            let min_y = ARENA_HEIGHT - PLAYER_HEIGHT * 0.5;
            let max_x = PLAYER_WIDTH * 0.5;
            let max_y = PLAYER_HEIGHT * 0.5;

            let trans_x = input.axis_value("horizontal").unwrap_or(0.0);
            let trans_y = input.axis_value("vertical").unwrap_or(0.0);

            let x = transf.translation().x;
            let y = transf.translation().y;

            // Set the x and y positions, ensuring the player is not off the
            // screen
            transf.set_translation_x((x + trans_x).min(min_x).max(max_x));
            transf.set_translation_y((y + trans_y).min(min_y).max(max_y));
        }
    }
}
