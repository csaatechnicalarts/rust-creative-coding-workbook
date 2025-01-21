#![allow(warnings)]

use raylib::prelude::*;
use std::collections::HashMap;
use std::process;

use crate::rangoli::RangoliTextPattern;

pub const SCREEN_WIDTH: i32 = 1280;
pub const SCREEN_HEIGHT: i32 = 1024;
pub const DEFAULT_FPS: u32 = 24;
pub const FONT_SIZE: f32 = 18.0;

//const ALPHA_DELIM: char = '-';
const ALPHA_WIDTH_PAD: f32 = 3.0;
const ALPHA_HEIGHT_PAD: f32 = 18.5;
const X_OFFSET_THRESHOLD: f32 = 2.0;
const TOP_OFFSET: f32 = 40.0;

#[derive(Debug, Clone, Copy)]
struct AlphaToDisplay {
    alpha: char,
    coord: Vector2,
}

pub struct RLDriver<'p> {
    rl: &'p mut RaylibHandle,
    thread: &'p RaylibThread,
    fps: u32,
    font: &'p Font,
    // Generated text pattern owned by the rangoli module.
    rangoli_text: &'p RangoliTextPattern,
    // Glyph representation of the rangoli pattern to display.
    rangoli_disp: Vec<Vec<AlphaToDisplay>>,
    // X-offset of a character of the given font set.
    alpha_offsets: HashMap<char, f32>,
}

impl AlphaToDisplay {
    fn new(
        c: char,
        line_index: usize,
        mid_index: usize,
        char_index: usize,
        alpha_offsets: &HashMap<char, f32>,
        max_alpha_offset: f32,
    ) -> Self {
        let mut alpha_coord = Vector2::default();

        // For narrow letters such as 'i', 'j' or 't', fudge their x-coordinate
        // to display the glyph closer to the center of their display cell,
        // at the least by X_OFFSET_THRESHOLD. This is purely for aesthetic effect
        // and may not work well for all fonts.

        let x_offset = alpha_offsets.get(&c);

        if let Some(val) = x_offset {
            if char_index == mid_index {
                alpha_coord.x = (SCREEN_WIDTH as f32 / 2.0);
            } else if char_index > mid_index {
                alpha_coord.x = (SCREEN_WIDTH as f32 / 2.0)
                    + ((char_index - mid_index) as f32 * (max_alpha_offset + 2.0))
                    + ((char_index - mid_index) as f32 * ALPHA_WIDTH_PAD);
            } else {
                alpha_coord.x = (SCREEN_WIDTH as f32 / 2.0)
                    - ((mid_index - char_index) as f32 * (max_alpha_offset + 2.0))
                    - ((mid_index - char_index) as f32 * ALPHA_WIDTH_PAD);
            }

            if *val <= X_OFFSET_THRESHOLD {
                alpha_coord.x += X_OFFSET_THRESHOLD;
            }
        } else {
            panic!("Error: alpha_offset.get(&c) returned None!");
        }

        alpha_coord.y = TOP_OFFSET + (ALPHA_HEIGHT_PAD * (line_index as f32));

        Self {
            alpha: c,
            coord: alpha_coord,
        }
    }
}

impl<'p> RLDriver<'p> {
    pub fn build(
        rl: &'p mut RaylibHandle,
        thread: &'p RaylibThread,
        font: &'p Font,
        rangoli_text: &'p mut RangoliTextPattern,
    ) -> RLDriver<'p> {
        let (max_alpha_offset, alpha_offsets) = RLDriver::calc_alpha_offsets(&rl);
        let mut outer_vec: Vec<Vec<AlphaToDisplay>> = Vec::new();
        let (rangoli_pattern, _) = rangoli_text.get_rangoli_text();

        for (line_index, r_line) in rangoli_pattern.iter().enumerate() {
            let mut inner_vec: Vec<AlphaToDisplay> = Vec::new();

            // The middle character is always 'a', the pivot of the range of characters
            // incrementing to the left and right respectively:
            // i.e. ["a", "b-a-b", "c-b-a-b-c"] for an n=3 rangoli pattern.

            let mid_index: usize = r_line.chars().count() / 2;
            for char_index in 0..r_line.chars().count() {
                // A line in a rangoli pattern always has an odd number of alphabets,
                // each one indexed by an even number. Odd indices always refer to
                // the delimeter character, '-' in "c-b-a-b-c" for example.
                //
                // We could have done the following, but modulo arithmetic is always
                // faster than a memory lookup and logical comparison combo operation:
                // if r_line.chars().nth(j as usize) != Some(ALPHA_DELIM) {...}

                if char_index % 2 == 0 {
                    let alpha_char = r_line.chars().nth(char_index);

                    if let Some(c) = alpha_char {
                        inner_vec.push(AlphaToDisplay::new(
                            c,
                            line_index,
                            mid_index,
                            char_index,
                            &alpha_offsets,
                            max_alpha_offset,
                        ));
                    } else {
                        panic!("Error: r_line.chars().nth(char_index) yielded None!");
                    }
                }
            }

            outer_vec.push(inner_vec);
        }

        // The outer_vec represents the upper half of the rangoli display.
        // rangoli_mirrored contains both the upper and lower halves.

        let outer_vec_len = outer_vec.len();
        let mut rangoli_mirrored = outer_vec.clone();

        let mut line_ctr = 1.0;
        for inner_vec in (&outer_vec[..(outer_vec_len - 1)]).into_iter().rev() {
            rangoli_mirrored.push(RLDriver::mirror_inner_vec(&line_ctr, &inner_vec));
            line_ctr += 1.0;
        }

        RLDriver {
            rl,
            thread,
            font,
            fps: DEFAULT_FPS,
            rangoli_text,
            rangoli_disp: rangoli_mirrored,
            alpha_offsets,
        }
    }

    fn mirror_inner_vec(line_ctr: &f32, inner_vec: &Vec<AlphaToDisplay>) -> Vec<AlphaToDisplay> {
        let mut retVal = Vec::new();

        for a2d_old in inner_vec {
            let a2d_new = AlphaToDisplay {
                alpha: a2d_old.alpha,
                coord: Vector2::new(
                    a2d_old.coord.x,
                    (a2d_old.coord.y + (2.0 * line_ctr * ALPHA_HEIGHT_PAD)),
                ),
            };

            retVal.push(a2d_new);
        }

        retVal
    }

    fn calc_alpha_offsets(rl: &RaylibHandle) -> (f32, HashMap<char, f32>) {
        let ascii_lower: [char; 26 as usize] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];
        let mut ret_val: HashMap<char, f32> = HashMap::new();
        let mut max_alpha_offset: f32 = 0.0;

        // A temporary variable is required for a char conversion because
        // a char in Rust is a 4 byte UTF8, unlike ASCII.

        let mut tmp = [0u8; 4];
        for i in 0..26 {
            let mut x_offset = (rl
                .measure_text(ascii_lower[i].encode_utf8(&mut tmp), FONT_SIZE as i32)
                / 2) as f32;
            ret_val.insert(ascii_lower[i], x_offset);
            if x_offset > max_alpha_offset {
                max_alpha_offset = x_offset;
            }
        }

        (max_alpha_offset, ret_val)
    }

    pub fn run(&mut self) {
        while !self.rl.window_should_close() {
            // *******************
            // Raylib logic block.
            // *******************

            // ******************
            // Raylib draw block.
            // ******************

            let mut d = self.rl.begin_drawing(&self.thread);

            d.clear_background(Color::SADDLEBROWN);

            for r_line in &self.rangoli_disp {
                for r_char in r_line {
                    let mut tmp = [0u8; 4];

                    d.draw_text_ex(
                        &self.font,
                        r_char.alpha.encode_utf8(&mut tmp),
                        r_char.coord,
                        FONT_SIZE,
                        1.0,
                        Color::DARKKHAKI,
                    );
                }
            }
        }
    }
}
