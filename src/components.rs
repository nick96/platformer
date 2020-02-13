use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, SpriteRender, SpriteSheet},
};

use crate::consts::{ARENA_HEIGHT, ARENA_WIDTH};

pub struct Player;

impl Player {
    pub fn new() -> Player {
        Player {}
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_player(world: &mut World, ssh: Handle<SpriteSheet>) {
    let mut local_transf = Transform::default();
    local_transf.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
    let sprite_render = SpriteRender {
        sprite_sheet: ssh.clone(),
        sprite_number: 1,
    };
    world
        .create_entity()
        .with(sprite_render)
        .with(Player::new())
        .with(local_transf)
        .build();
}

pub fn init_camera(world: &mut World) {
    let mut tranf = Transform::default();
    tranf.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(tranf)
        .build();
}
