#![allow(warnings)]

use raylib::prelude::*;
use std::collections::HashMap;

pub const SCREEN_WIDTH: i32 = 800;
pub const SCREEN_HEIGHT: i32 = 450;
pub const DEFAULT_FPS: u32 = 24;
pub const FONT_SIZE: i32 = 20;

pub struct RLDriver<'pattern_lt> {
    rl: &'pattern_lt mut RaylibHandle,
    thread: &'pattern_lt RaylibThread,
    fps: u32,
    font: &'pattern_lt Font,
    // Generated text pattern owned by the rangoli module.
    rangoli_pattern: &'pattern_lt Vec<String>,     
    // X- and Y-coordinate of each character in the rangoli pattern.
    rangoli_pos: &'pattern_lt Vec<Vector2>,
    // X-offset of a character of the given font set.
    alpha_offsets: HashMap<char, f32>,
}

impl<'pattern_lt> RLDriver<'pattern_lt> {
    pub fn build(rl: &'pattern_lt mut RaylibHandle, thread: &'pattern_lt RaylibThread, 
                 font: &'pattern_lt Font, rangoli_pattern: &'pattern_lt Vec<String>, 
                 rangoli_pos: &'pattern_lt Vec<Vector2>) -> RLDriver<'pattern_lt> {

        let alpha_offsets = RLDriver::calc_alpha_offsets(&rl);
        println!("\n{:#?}", alpha_offsets);

        RLDriver{
           rl,
           thread,
           font,
           fps: DEFAULT_FPS,
           rangoli_pattern,
           rangoli_pos,
           alpha_offsets,
        }
    }

    fn calc_alpha_offsets(rl: &RaylibHandle) -> HashMap<char, f32> {
        let ascii_lower: [char; 26 as usize] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 
            'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 
            'y', 'z',
        ];

        let mut retVal: HashMap<char, f32> = HashMap::new();

        // A temporary variable is required for a char conversion because 
        // a char in Rust is a 4 byte UTF8, unlike ASCII.
        let mut tmp = [0u8; 4];
        for i in 0..26 {
            let x_offset = (rl.measure_text(ascii_lower[i].encode_utf8(&mut tmp), FONT_SIZE) / 2) as f32;  
            retVal.insert(ascii_lower[i], x_offset);
        }

        retVal
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

            for i in 0..self.rangoli_pattern.len() {
                let r_line = self.rangoli_pattern.get(i).unwrap();
                d.draw_text_ex(
                    &self.font, 
                    r_line, 
                    self.rangoli_pos[i as usize],
                    20.0, 
                    1.0, 
                    Color::GRAY
                );
            }
        }    
    }
}
