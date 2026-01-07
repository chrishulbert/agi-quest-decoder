// This is responsible for parsing 'views' which are bitmap sprites/animations.

use anyhow::Result;

use crate::png;
use crate::palette::{PALETTE, TRANSPARENT};

pub struct View {
    pub loops: Vec<Loop>,
}

impl View {
    pub fn parse(data: &[u8]) -> Result<View> {
        let _step_size = data[0];
        let _cycle_time = data[1];
        let loop_count = data[2];
        let description_position = (data[3] as usize) + ((data[4] as usize) << 8);
        if description_position != 0 {
            let description = parse_asciiz_string(&data[description_position..])?;
            println!("Found view description: {}", description);
        }
        let mut loops: Vec<Loop> = Vec::with_capacity(loop_count as usize);
        for i in 0..loop_count {
            // Read the position.
            let offset = 5 + (i as usize) * 2;
            let position = (data[offset] as usize) + ((data[offset + 1] as usize) << 8);
            // Read the loop.
            let loop_data = &data[position..];
            let view_loop = Loop::parse(loop_data)?;
            loops.push(view_loop);
        }
        Ok(View{ loops })
    }
}

pub struct Loop {
    pub cels: Vec<Cel>,
}
impl Loop {
    fn parse(data: &[u8]) -> Result<Loop> {
        let cel_count = data[0] as usize;
        let positions_data_a = &data[1..];
        let positions_data = &positions_data_a[0..(cel_count*2)];
        let positions: Vec<usize> = positions_data.chunks_exact(2).map(parse_2_byte_le).collect();
        let cels: Vec<Cel> = positions.iter().map(|&p| {
            Cel::parse(&data[p..])
        }).collect();
        Ok(Loop { cels })
    }

    // It's eligible to be an animation if all cels are the same size and there are 2+.
    pub fn is_animation(&self) -> bool {
        if self.cels.len() < 2 { return false }
        for c in &self.cels {
            if c.width != self.cels[0].width { return false }
            if c.height != self.cels[0].height { return false }
        }
        true
    }

    pub fn as_apng(&self) -> Vec<u8> {
        let frames: Vec<Vec<u32>> = self.cels.iter().map(|c| c.as_rgbas()).collect();
        png::apng_data(self.cels[0].width as u32, self.cels[0].height as u32, &frames)
    }
}

pub struct Cel {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>, // EGA palette indexes.
}
impl Cel {
    fn parse(data: &[u8]) -> Cel {
        let width = data[0] as usize;
        let height = data[1] as usize;
        let transparency_mirroring = data[2];
        let mirroring = transparency_mirroring >> 4;
        let _is_mirrored = mirroring & 0x8 != 0;
        // if is_mirrored {
        //     println!("Found mirrored cel!");
        // }
        let transparent = transparency_mirroring & 0xf;
        let image_source_data = &data[3..];
        let mut pixels: Vec<u8> = Vec::with_capacity(width * height);
        'outer: for b in image_source_data {
            if *b == 0 { // End of line.
                let pixels_filled_in_this_row = pixels.len() % width;
                if pixels_filled_in_this_row == 0 {
                    // Do nothing, we've just filled a full row, so nothing left to fill in.
                } else {
                    // We filled half a row, so fill the rest with transparent.
                    let remaining = width - pixels_filled_in_this_row;
                    for _ in 0..remaining {
                        pixels.push(TRANSPARENT);
                        if pixels.len() >= width * height {
                            break 'outer;
                        }
                    }
                }
            } else {
                let raw_colour = b >> 4;
                let colour = if raw_colour == transparent { TRANSPARENT } else { raw_colour };
                let count = b & 0xf;
                for _ in 0..count {
                    pixels.push(colour);
                    if pixels.len() >= width * height {
                        break 'outer;
                    }
                }
            }
        }
        while pixels.len() < width * height {
            pixels.push(TRANSPARENT);
        }
        Cel { width, height, pixels }
    }

    pub fn as_png(&self) -> Vec<u8> {
        png::png_data(self.width as u32, self.height as u32, &self.as_rgbas())
    }

    fn as_rgbas(&self) -> Vec<u32> {
        self.pixels.iter().map(|&p| { PALETTE[p as usize] }).collect()
    }
}

fn parse_2_byte_le(data: &[u8]) -> usize {
    (data[0] as usize) + ((data[1] as usize) << 8)
}

fn parse_asciiz_string(data: &[u8]) -> std::result::Result<String, std::string::FromUtf8Error> {
    let position = data.iter().position(|&c| c==0);
    let slice = match position {
        Some(i) => &data[0..i],
        None => data,
    };
    String::from_utf8(slice.to_vec())
}
