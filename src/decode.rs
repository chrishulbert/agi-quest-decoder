// This is responsible for coordinating the whole decoding process.

use anyhow::Result;

use crate::directories;
use crate::picture;
use crate::resources;
use crate::volumes;
use crate::view;
use crate::renderer;

pub fn decode(path: &str) -> Result<()> {
    let directories = directories::Directories::read(path)?;
    println!("Logic entries: {}", directories.logic.entries.len());
    println!("Sounds entries: {}", directories.sounds.entries.len());
    println!("Views entries: {}", directories.views.entries.len());
    println!("Pictures entries: {}", directories.pictures.entries.len());
    let volumes = volumes::read(path)?;
    let resources = resources::Resources::parse(&volumes, &directories)?;
    
    for (pi, resource) in resources.pictures.iter().enumerate() {
        if resource.is_empty() { continue }
        let picture = picture::Picture::parse(&resource);
        let name = format!("Output.picture.{}.static.png", pi);
        let png = renderer::png_from_picture(&picture);
        std::fs::write(name, png)?;
    }

    for (vi, resource) in resources.views.iter().enumerate() {
        if resource.is_empty() { continue }
        let view = view::View::parse(&resource)?;
        for (li, l) in view.loops.iter().enumerate() {
            if renderer::is_animation(l) {
                // Animated.
                let name = format!("Output.view.{}.{}.animation.png", vi, li);
                let png = renderer::apng_from_loop(l);
                std::fs::write(name, png)?;
            } else {
                // Not animated.
                for (ci, c) in l.cels.iter().enumerate() {
                    let name = format!("Output.view.{}.{}.{}.static.png", vi, li, ci);
                    let png = renderer::png_from_cel(c);
                    std::fs::write(name, png)?;
                }
            }
        }
    }
    Ok(())
}
