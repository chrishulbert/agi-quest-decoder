// This is responsible for coordinating the whole decoding process.

use anyhow::Result;

use crate::directories;
use crate::resources;
use crate::volumes;

pub fn decode(path: &str) -> Result<()> {
    let directories = directories::Directories::read(path)?;
    println!("Logic entries: {}", directories.logic.entries.len());
    println!("Sounds entries: {}", directories.sounds.entries.len());
    println!("Views entries: {}", directories.views.entries.len());
    println!("Pictures entries: {}", directories.pictures.entries.len());
    let volumes = volumes::read(path)?;
    let resources = resources::Resources::parse(&volumes, &directories)?;
    for x in resources.views.iter().take(20) {
        println!("{}", x.len());
    }
    Ok(())
}
