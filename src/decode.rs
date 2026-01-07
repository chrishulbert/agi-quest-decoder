// This is responsible for coordinating the whole decoding process.

use anyhow::Result;

use crate::directories;
use crate::resources;
use crate::volumes;
use crate::view;

pub fn decode(path: &str) -> Result<()> {
    let directories = directories::Directories::read(path)?;
    println!("Logic entries: {}", directories.logic.entries.len());
    println!("Sounds entries: {}", directories.sounds.entries.len());
    println!("Views entries: {}", directories.views.entries.len());
    println!("Pictures entries: {}", directories.pictures.entries.len());
    let volumes = volumes::read(path)?;
    let resources = resources::Resources::parse(&volumes, &directories)?;
    for (vi, resource) in resources.views.iter().enumerate() {
        if resource.is_empty() { continue }
        let view = view::View::parse(&resource)?;
        for (li, l) in view.loops.iter().enumerate() {
            if l.is_animation() {
                // Animated.
                let name = format!("Output.view.{}.{}.animation.png", vi, li);
                let png = l.as_apng();
                std::fs::write(name, png)?;
            } else {
                // Not animated.
                for (ci, c) in l.cels.iter().enumerate() {
                    let name = format!("Output.view.{}.{}.{}.static.png", vi, li, ci);
                    let png = c.as_png();
                    std::fs::write(name, png)?;
                }
            }
        }
    }
    Ok(())
}
