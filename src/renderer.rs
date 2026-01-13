// This is responsible for rendering views to PNGs.
// This deals with the aspect ratio issues and expanding to a visible size.

use crate::view::{Loop, Cel};
use crate::{picture, png};
use crate::palette::{PALETTE, TRANSPARENT};

// The game is originally rendered at 320x200 on a 4:3 screen, so pixels are 1.2x higher than wide.
// Plus each game pixel is rendered 2 pixels wide, so we the only 'true' ratios are 6x10 or 3x5.
// const MULTIPLIER: usize = 2.5 or 5; // Minimum number at which the aspect ratio correct works.
// const HEIGHT_MULTIPLIER: usize = MULTIPLIER * 240 / 200; // Aspect ratio correction.
// const WIDTH_MULTIPLIER: usize = MULTIPLIER * 2;
const HEIGHT_MULTIPLIER: usize = 3;
const WIDTH_MULTIPLIER: usize = 5;

// It's eligible to be an animation even if sizes are different.
// Padding is added to the top and right, which seems to align cels nicely on space quest.
pub fn is_animation(viewloop: &Loop) -> bool {
    viewloop.cels.len() >= 2
}

pub fn apng_from_loop(viewloop: &Loop) -> Vec<u8> {
    // Get max height.
    let width = viewloop.cels.iter().map(|c| c.width).max().unwrap();
    let height = viewloop.cels.iter().map(|c| c.height).max().unwrap();
    let frames: Vec<Vec<u32>> = viewloop.cels.iter()
        .map(|c| pad_cel(c, width, height))
        .map(|c| rgbas_from_cel(&c))
        .collect();
    png::apng_data(
        width * WIDTH_MULTIPLIER,
        height * HEIGHT_MULTIPLIER,
        &frames)
}

pub fn png_from_cel(cel: &Cel) -> Vec<u8> {
    png::png_data(
        cel.width * WIDTH_MULTIPLIER,
        cel.height * HEIGHT_MULTIPLIER,
        &rgbas_from_cel(cel))
}

pub fn png_from_picture(picture: &picture::Picture) -> Vec<u8> {
    let cel: Cel = Cel {
        width: picture::WIDTH,
        height: picture::HEIGHT,
        pixels: picture.picture.clone(),
    };
    png_from_cel(&cel)
}

// Increase the width/height of a cel.
fn pad_cel(cel: &Cel, width: usize, height: usize) -> Cel {
    if cel.width == width && cel.height == height { return cel.clone(); }
    let mut pixels: Vec<u8> = Vec::with_capacity(width * height);
    // Pad the height:
    let extra_height = height - cel.height;
    let extra_pixels_top = extra_height * width;
    pixels.extend(vec![TRANSPARENT; extra_pixels_top]);
    // Pad the width:
    let extra_width = width - cel.width;
    for row in cel.pixels.chunks_exact(cel.width) {
        pixels.extend_from_slice(row);
        for _ in 0..extra_width {
            pixels.push(TRANSPARENT);
        }
    }
    Cel { width, height, pixels }
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
