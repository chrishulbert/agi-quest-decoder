mod decode;
mod directories;
mod volumes;
mod resources;
mod view;
mod palette;
mod png;
mod renderer;
mod picture;
mod picture_splitter;
mod xbrz;

use anyhow::Result;

fn main() -> Result<()> {
    println!("-=[ AGI Quest Decoder ]=-");

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage:");
        println!("agi-quest-decoder /Path/To/SQ2");
    } else {
        decode::decode(&args[1])?;
    }
    Ok(())
}
