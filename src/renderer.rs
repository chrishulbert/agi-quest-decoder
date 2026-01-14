// This is responsible for rendering views to PNGs.
// This deals with the aspect ratio issues and expanding to a visible size.

use crate::view::{Loop, Cel};
use crate::{picture, png};
use crate::palette::{PALETTE, TRANSPARENT};
use crate::xbrz;

// The game is originally rendered at 320x200 on a 4:3 screen, so pixels are 1.2x higher than wide.
// Plus each game pixel is rendered 2 pixels wide, so we the only 'true' ratios are 6x10 or 3x5.
// const MULTIPLIER: usize = 2.5 or 5; // Minimum number at which the aspect ratio correct works.
// const HEIGHT_MULTIPLIER: usize = MULTIPLIER * 240 / 200; // Aspect ratio correction.
// const WIDTH_MULTIPLIER: usize = MULTIPLIER * 2;
const WIDTH_MULTIPLIER: usize = 5;
const HEIGHT_MULTIPLIER: usize = 3;

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

// This converts an unscaled cel to scaled rgbas.
fn rgbas_from_cel(cel: &Cel) -> Vec<u32> {
    let unscaled_rgbas: Vec<u32> = cel.pixels.iter().map(|p| PALETTE[*p as usize]).collect();
    // Scale using xbrz - this only really works if the multipliers are 5 and 3:
    // Go to 5x5:
    let scaled_square = xbrz::scale(WIDTH_MULTIPLIER as u8, &unscaled_rgbas, cel.width as u32, cel.height as u32);
    // Now descale to 5x3:
    let scaled_width = cel.width * WIDTH_MULTIPLIER;
    let chunk_size_for_original_row = scaled_width * WIDTH_MULTIPLIER;
    let mut scaled_aspect: Vec<u32> = Vec::with_capacity(cel.width * WIDTH_MULTIPLIER * cel.height * HEIGHT_MULTIPLIER);
    for chunk in scaled_square.chunks_exact(chunk_size_for_original_row) {
        let row_0 = &chunk[0..scaled_width];
        let row_1 = &chunk[scaled_width..(scaled_width*2)];
        let row_2 = &chunk[(scaled_width*2)..(scaled_width*3)];
        let row_3 = &chunk[(scaled_width*3)..(scaled_width*4)];
        let row_4 = &chunk[(scaled_width*4)..];
        for (&a, &b) in row_0.iter().zip(row_1) {
            scaled_aspect.push(interpolate_rgba(a, b));
        }
        scaled_aspect.extend_from_slice(row_2);
        for (&a, &b) in row_3.iter().zip(row_4) {
            scaled_aspect.push(interpolate_rgba(a, b));
        }
    }
    scaled_aspect

    // Nearest-neighbour scaling:
    // let mut rgbas: Vec<u32> = Vec::with_capacity(cel.width * cel.height * WIDTH_MULTIPLIER * HEIGHT_MULTIPLIER);
    // for row in cel.pixels.chunks_exact(cel.width) {
    //     for _ in 0..HEIGHT_MULTIPLIER {
    //         for p in row {
    //             let rgba = PALETTE[*p as usize];
    //             for _ in 0..WIDTH_MULTIPLIER {
    //                 rgbas.push(rgba);
    //             }
    //         }
    //     }
    // }
    // rgbas
}

fn interpolate_rgba(x: u32, y: u32) -> u32 {
    let r1 = x >> 24;
    let g1 = (x >> 16) & 0xff;
    let b1 = (x >> 8) & 0xff;
    let a1 = x & 0xff;
    let r2 = y >> 24;
    let g2 = (y >> 16) & 0xff;
    let b2 = (y >> 8) & 0xff;
    let a2 = y & 0xff;
    let r = (r1 + r2) / 2;
    let g = (g1 + g2) / 2;
    let b = (b1 + b2) / 2;
    let a = (a1 + a2) / 2;
    (r << 24) + (g << 16) + (b << 8) + a
}