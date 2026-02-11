// This is responsible for rendering views to PNGs.
// This deals with the aspect ratio issues and expanding to a visible size.

use crate::view::{Loop, Cel};
use crate::{picture, png};
use crate::palette::{PALETTE, TRANSPARENT};
use crate::scalefx;

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
    
    // Scale using ScaleFX - this only really works if the multipliers are 5 and 3:
    assert!(WIDTH_MULTIPLIER == 5 && HEIGHT_MULTIPLIER == 3);

    // Go to 9x9:
    let (_sfx_width, sfx_height, sfx_pixels) = scalefx::scale9x(cel.width, cel.height, &unscaled_rgbas);

    // Shrink horizontally to 5x:
    let mut scaled_horizontal: Vec<u32> = Vec::with_capacity(cel.width * WIDTH_MULTIPLIER * sfx_height);
    for chunk in sfx_pixels.chunks_exact(9) {
        scaled_horizontal.push(interpolate_rgba(chunk[0], chunk[1]));
        scaled_horizontal.push(interpolate_rgba(chunk[2], chunk[3]));
        scaled_horizontal.push(chunk[4]); // Would it look nicer if 4 was interpolated with both 3 and 5?
        scaled_horizontal.push(interpolate_rgba(chunk[5], chunk[6]));
        scaled_horizontal.push(interpolate_rgba(chunk[7], chunk[8]));
    }

    // Scale vertically from 9x to 3x, by splitting into each 3 row triplet, and outputting 1 row:
    let mut scaled_aspect: Vec<u32> = Vec::with_capacity(cel.width * WIDTH_MULTIPLIER * cel.height * HEIGHT_MULTIPLIER);
    let scaled_row_size = cel.width * WIDTH_MULTIPLIER;
    for triplet in scaled_horizontal.chunks_exact(scaled_row_size * 3) {
        // Split this triplet of scaled rows into each of the scaled rows:
        let row_top = &triplet[..scaled_row_size];
        let row_mid = &triplet[scaled_row_size..scaled_row_size*2];
        let row_bot = &triplet[scaled_row_size*2..];
        // Interpolate them, so the middle row gets interpolated with both the top and bottom, but half-weighted each time.
        for ((a, b), c) in row_top.iter().zip(row_mid).zip(row_bot) {
            scaled_aspect.push(interpolate_rgba_3(*a, *b, *c));
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

fn interpolate_rgba_3(x: u32, y: u32, z: u32) -> u32 {
    let r1 = x >> 24;
    let g1 = (x >> 16) & 0xff;
    let b1 = (x >> 8) & 0xff;
    let a1 = x & 0xff;
    let r2 = y >> 24;
    let g2 = (y >> 16) & 0xff;
    let b2 = (y >> 8) & 0xff;
    let a2 = y & 0xff;
    let r3 = z >> 24;
    let g3 = (z >> 16) & 0xff;
    let b3 = (z >> 8) & 0xff;
    let a3 = z & 0xff;
    let r = (r1 + r2 + r3) / 3;
    let g = (g1 + g2 + g3) / 3;
    let b = (b1 + b2 + b3) / 3;
    let a = (a1 + a2 + a3) / 3;
    (r << 24) + (g << 16) + (b << 8) + a
}