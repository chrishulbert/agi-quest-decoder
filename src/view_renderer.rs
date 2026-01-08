// This is responsible for rendering views to PNGs.
// This deals with the aspect ratio issues and expanding to a visible size.

use crate::view::{Loop, Cel};
use crate::png;
use crate::palette::PALETTE;

const HEIGHT_MULTIPLIER: usize = 4;
const WIDTH_MULTIPLIER: usize = HEIGHT_MULTIPLIER * 2;

// It's eligible to be an animation if all cels are the same size and there are 2+.
pub fn is_animation(viewloop: &Loop) -> bool {
    if viewloop.cels.len() < 2 { return false }
    for c in &viewloop.cels {
        if c.width  != viewloop.cels[0].width  { return false }
        if c.height != viewloop.cels[0].height { return false }
    }
    true
}

pub fn apng_from_loop(viewloop: &Loop) -> Vec<u8> {
    let frames: Vec<Vec<u32>> = viewloop.cels.iter().map(rgbas_from_cel).collect();
    png::apng_data(
        viewloop.cels[0].width * WIDTH_MULTIPLIER,
        viewloop.cels[0].height * HEIGHT_MULTIPLIER,
        &frames)
}

pub fn png_from_cel(cel: &Cel) -> Vec<u8> {
    png::png_data(
        cel.width * WIDTH_MULTIPLIER,
        cel.height * HEIGHT_MULTIPLIER,
        &rgbas_from_cel(cel))
}

fn rgbas_from_cel(cel: &Cel) -> Vec<u32> {
    let mut rgbas: Vec<u32> = Vec::with_capacity(cel.width * cel.height * WIDTH_MULTIPLIER * HEIGHT_MULTIPLIER);
    for row in cel.pixels.chunks_exact(cel.width) {
        for _ in 0..HEIGHT_MULTIPLIER {
            for p in row {
                let rgba = PALETTE[*p as usize];
                for _ in 0..WIDTH_MULTIPLIER {
                    rgbas.push(rgba);
                }
            }
        }
    }
    rgbas
}
