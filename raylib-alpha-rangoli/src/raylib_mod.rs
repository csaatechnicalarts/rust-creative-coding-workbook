#![allow(warnings)]

use raylib::prelude::*;
use std::collections::HashMap;

pub const SCREEN_WIDTH: i32 = 1280;
pub const SCREEN_HEIGHT: i32 = 800;
pub const DEFAULT_FPS: u32 = 24;
pub const FONT_SIZE: f32 = 18.0;

const ALPHA_DELIM: char = '-';
const ALPHA_WIDTH_PAD: f32 = 3.0;
const ALPHA_HEIGHT_PAD: f32 = 18.5;

#[derive(Debug)]
struct AlphaToDisplay {
    alpha: char,
    coord: Vector2,
}

pub struct RLDriver<'pattern_lt> {
    rl: &'pattern_lt mut RaylibHandle,
    thread: &'pattern_lt RaylibThread,
    fps: u32,
    font: &'pattern_lt Font,
    // Generated text pattern owned by the rangoli module.
    rangoli_pattern: &'pattern_lt Vec<String>,
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
    ) -> AlphaToDisplay {
        let mut alpha_coord = Vector2::default();

        if char_index == mid_index {
            alpha_coord.x = SCREEN_WIDTH as f32 / 2.0;
        } else if char_index > mid_index {
            alpha_coord.x = (SCREEN_WIDTH as f32 / 2.0)
                + ((char_index - mid_index) as f32 * (max_alpha_offset + 2.0))
                + ((char_index - mid_index) as f32 * ALPHA_WIDTH_PAD);
            //alpha_offsets.get(&c).unwrap();
        } else {
            alpha_coord.x = (SCREEN_WIDTH as f32 / 2.0)
                - ((mid_index - char_index) as f32 * (max_alpha_offset + 2.0))
                - ((mid_index - char_index) as f32 * ALPHA_WIDTH_PAD);
            //alpha_offsets.get(&c).unwrap();
        }
        alpha_coord.y = 60.0 + (ALPHA_HEIGHT_PAD * (line_index as f32));

        let mut retVal = AlphaToDisplay {
            alpha: c,
            coord: alpha_coord,
        };

        retVal
    }
}

impl<'pattern_lt> RLDriver<'pattern_lt> {
    pub fn build(
        rl: &'pattern_lt mut RaylibHandle,
        thread: &'pattern_lt RaylibThread,
        font: &'pattern_lt Font,
        rangoli_pattern: &'pattern_lt Vec<String>,
    ) -> RLDriver<'pattern_lt> {
        let (max_alpha_offset, alpha_offsets) = RLDriver::calc_alpha_offsets(&rl);
        //println!("\n{:#?}\nmax_alpha_offset: {:.2}", alpha_offsets, max_alpha_offset);

        let mut outer_vec: Vec<Vec<AlphaToDisplay>> = Vec::new();
        //for r_line in rangoli_pattern {
        for line_index in 0..rangoli_pattern.len() {
            let r_line = &rangoli_pattern[line_index];
            let mut inner_vec: Vec<AlphaToDisplay> = Vec::new();

            // The middle character is always 'a', the pivot of the range of characters
            // incrementing to the left and right respectively:
            // i.e. ["a", "b-a-b", "c-b-a-b-c"] for an n=3 rangoli pattern.

            let mid_index: usize = r_line.len() / 2;
            for char_index in 0..r_line.len() {
                // A line in a rangoli pattern always has an odd number of alphabets,
                // each one indexed by an even number. Odd indices always refer to
                // the delimeter character, '-' in "c-b-a-b-c" for example.
                //
                // We could have done the following, but modulo arithmetic is always
                // faster than a memory lookup and logical comparison combo operation:
                // if r_line.chars().nth(j as usize) != Some(ALPHA_DELIM) {...}

                if char_index % 2 == 0 {
                    inner_vec.push(AlphaToDisplay::new(
                        r_line.chars().nth(char_index).unwrap(),
                        line_index,
                        mid_index,
                        char_index,
                        &alpha_offsets,
                        max_alpha_offset,
                    ));
                }
            }

            outer_vec.push(inner_vec);
        }
        println!("{:#?}", outer_vec);

        RLDriver {
            rl,
            thread,
            font,
            fps: DEFAULT_FPS,
            rangoli_pattern,
            rangoli_disp: outer_vec,
            alpha_offsets,
        }
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
            let x_offset = (rl.measure_text(ascii_lower[i].encode_utf8(&mut tmp), FONT_SIZE as i32)
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

            d.clear_background(Color::GAINSBORO);

            for r_line in &self.rangoli_disp {
                for r_char in r_line {
                    let mut tmp = [0u8; 4];

                    d.draw_text_ex(
                        &self.font,
                        r_char.alpha.encode_utf8(&mut tmp),
                        r_char.coord,
                        FONT_SIZE,
                        1.0,
                        Color::GRAY,
                    );
                }
            }
        }
    }
}
