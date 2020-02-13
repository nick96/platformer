// Build script to check the ron sprite sheets at build time
// use amethyst::renderer::sprite::{SpriteGrid, SpriteList};

use std::env;
use std::fs;

fn check_dir(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.path();
        if file_name.is_dir() {
            check_dir(&file_name)?;
        } else if file_name.extension().unwrap().to_str().unwrap() == "ron" {
            let contents = fs::read_to_string(file_name)?;
            let _ = ron::de::from_str::<amethyst_rendy::sprite::Sprites>(&contents)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=assets/textures/*.ron");
    let curr_dir = env::current_dir()?;
    check_dir(&curr_dir.join("assets"))?;
    Ok(())
}
