use crate::palette;
use crate::picture_splitter;

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 168;

pub struct Picture {
    pub picture: Vec<u8>,
}

impl Picture {
    pub fn parse(data: &[u8]) -> Picture {
        let actions = picture_splitter::split(data);
        let picture = draw(&actions);
        Picture { picture }
    }
}

fn draw(actions: &[picture_splitter::ActionArguments]) -> Vec<u8> {
    let mut picture: Vec<u8> = vec![palette::WHITE; WIDTH * HEIGHT];
    let mut is_drawing = false;
    let mut colour = palette::WHITE;
    let mut pen_is_splatter = false;
    for a in actions {
        match a.action {
            picture_splitter::Action::ChangePictureColourAndEnablePictureDraw => {
                is_drawing = true;
                colour = a.arguments[0];
            }
            picture_splitter::Action::DisablePictureDraw => {
                is_drawing = false;
            }
            picture_splitter::Action::ChangePriorityColourAndEnablePriorityDraw => {
                // I'm not doing anything with the 'priority' stuff.
            }
            picture_splitter::Action::DisablePriorityDraw => {
                // I'm not doing anything with the 'priority' stuff.
            }
            picture_splitter::Action::DrawAYCorner => {
                if is_drawing {
                    draw_corner(&mut picture, colour, &a.arguments, true);
                }
            }
            picture_splitter::Action::DrawAnXCorner => {
                if is_drawing {
                    draw_corner(&mut picture, colour, &a.arguments, false);
                }
            }
            picture_splitter::Action::AbsoluteLines => {
                if is_drawing {
                    draw_absolute_lines(&mut picture, colour, &a.arguments);
                }
            }
            picture_splitter::Action::RelativeLines => {
                if is_drawing {
                    draw_relative_lines(&mut picture, colour, &a.arguments);
                }
            }
            picture_splitter::Action::Fill => {
                if is_drawing {
                    fill(&mut picture, colour, &a.arguments);
                }
            }
            picture_splitter::Action::ChangePenSizeAndStyle => {
                pen_is_splatter = a.arguments[0] & 0x20 != 0; // false = solid.
                let _pen_is_rect = a.arguments[0] & 0x10 != 0; // false = circular.
                let _pen_size = a.arguments[0] & 7;
            }
            picture_splitter::Action::PlotWithPen => {
                // To keep it simple since it's not used except SQ2 and even then not
                // often, splatter isn't drawn.
                if is_drawing && !pen_is_splatter {
                    plot(&mut picture, colour, &a.arguments);
                }
            }
            picture_splitter::Action::Unused => {}
            picture_splitter::Action::End => {}
        }
    }
    picture
}

fn draw_corner(picture: &mut [u8], colour: u8, arguments: &[u8], is_y_corner: bool) {
    let mut x = arguments[0] as usize;
    let mut y = arguments[1] as usize;
    let moves = &arguments[2..];
    let mut is_y = is_y_corner;
    for &byte in moves {
        let coordinate = byte as usize;
        if is_y {
            let range = if coordinate > y {
                y..=coordinate
            } else {
                coordinate..=y
            };
            for y in range {
                picture[y * WIDTH + x] = colour;
            }
            y = coordinate;
        } else {
            let range = if coordinate > x {
                x..=coordinate
            } else {
                coordinate..=x
            };
            for x in range {
                picture[y * WIDTH + x] = colour;
            }
            x = coordinate;
        }
        is_y = !is_y;
    }
}

fn draw_absolute_lines(picture: &mut [u8], colour: u8, arguments: &[u8]) {
    let mut x = arguments[0] as usize;
    let mut y = arguments[1] as usize;
    let remaining_arguments = &arguments[2..];
    let moves = remaining_arguments.chunks_exact(2);
    for coordinate in moves {
        let new_x = coordinate[0] as usize;
        let new_y = coordinate[1] as usize;
        draw_agi_line(picture, colour, x, y, new_x, new_y);
        x = new_x;
        y = new_y;
    }
}

fn draw_relative_lines(picture: &mut [u8], colour: u8, arguments: &[u8]) {
    let mut x = arguments[0] as usize;
    let mut y = arguments[1] as usize;
    let remaining = &arguments[2..];
    for value in remaining {
        let y_raw = (value & 7) as usize;
        let y_is_minus = value & 8 > 0;
        let x_raw = ((value >> 4) & 7) as usize;
        let x_is_minus = value & 0x80 > 0;
        let new_x = if x_is_minus { x - x_raw } else { x + x_raw };
        let new_y = if y_is_minus { y - y_raw } else { y + y_raw };
        draw_agi_line(picture, colour, x, y, new_x, new_y);
        x = new_x;
        y = new_y;
    }
}

fn fill(picture: &mut [u8], colour: u8, arguments: &[u8]) {
    for chunk in arguments.chunks_exact(2) {
        let x = chunk[0] as usize;
        let y = chunk[1] as usize;
        let mut queue: Vec<(usize, usize)> = vec![(x, y)];
        while let Some(xy) = queue.pop() {
            let x = xy.0; 
            let y = xy.1; 
            let offset = y * WIDTH + x;
            if picture[offset] != palette::WHITE { continue }
            picture[offset] = colour;
            if x > 0 { queue.push((x-1, y)); } // Left.
            if x < WIDTH-1 { queue.push((x+1, y)); } // Right.
            if y > 0 { queue.push((x, y-1)); } // Up.
            if y < HEIGHT-1 { queue.push((x, y+1)); } // Down.
        }
    }
}

fn plot(picture: &mut [u8], colour: u8, arguments: &[u8]) {
    for chunk in arguments.chunks_exact(2) {
        let x = chunk[0] as usize;
        let y = chunk[1] as usize;
        if x >= WIDTH || y >= HEIGHT { continue } // Out of bounds.
        let offset = y * WIDTH + x;
        picture[offset] = colour;
    }
}

fn draw_agi_line(picture: &mut [u8], colour: u8, x1: usize, y1: usize, x2: usize, y2: usize) {
    // https://www.agidev.com/articles/agispec/agispecs-7.html
    fn agi_round(value: f32, direction: f32) -> usize {
        if direction < 0. {
            if value - value.floor() <= 0.501 {
                value.floor() as usize
            } else {
                value.ceil() as usize
            }
        } else {
            if value - value.floor() < 0.499 {
                value.floor() as usize
            } else {
                value.ceil() as usize
            }
        }
    }
    let height = (y2 as isize) - (y1 as isize);
    let width = (x2 as isize) - (x1 as isize);
    let add_x: f32 = if height == 0 { 0. } else { (width as f32) / (height.abs() as f32) };
    let add_y: f32 = if width == 0 { 0. } else { (height as f32) / (width.abs() as f32) };
    let mut x = x1 as f32;
    let mut y = y1 as f32;
    if width.abs() > height.abs() {
      let add_x: f32 = if width == 0 { 0. } else { if width > 0 { 1. } else { -1. } };
      if x2 > x1 {
        while x < (x2 as f32) {
            picture[
                agi_round(y, add_y) * WIDTH +
                agi_round(x, add_x)
            ] = colour;
            x += add_x;
            y += add_y;
        }
      } else {
        while x > (x2 as f32) {
            picture[
                agi_round(y, add_y) * WIDTH +
                agi_round(x, add_x)
            ] = colour;
            x += add_x;
            y += add_y;
        }
      }
   } else {
      let add_y: f32 = if height == 0 { 0. } else { if height > 0 { 1. } else { -1. } };
      if y2 > y1 {
        while y < (y2 as f32) {
            picture[
                agi_round(y, add_y) * WIDTH +
                agi_round(x, add_x)
            ] = colour;
            x += add_x;
            y += add_y;
        }
      } else {
        while y > (y2 as f32) {
            picture[
                agi_round(y, add_y) * WIDTH +
                agi_round(x, add_x)
            ] = colour;
            x += add_x;
            y += add_y;
        }
      }
   }
    picture[y2 * WIDTH + x2] = colour;
}
