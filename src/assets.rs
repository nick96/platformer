use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    prelude::*,
    renderer::{formats::texture::ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

pub fn load_sprite_sheet(world: &mut World, path: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let store = world.read_resource::<AssetStorage<Texture>>();
        loader.load(format!("{}.png", path), ImageFormat::default(), (), &store)
    };
    let loader = world.read_resource::<Loader>();
    let store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("{}.ron", path),
        SpriteSheetFormat(texture_handle),
        (),
        &store,
    )
}
