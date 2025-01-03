#![allow(warnings)]

use raylib::prelude::*;

pub const SCREEN_WIDTH: i32 = 800;
pub const SCREEN_HEIGHT: i32 = 450;
pub const DEFAULT_FPS: u32 = 24;

pub struct RLDriver<'pattern_lt> {
    rl: &'pattern_lt mut RaylibHandle,
    thread: &'pattern_lt RaylibThread,
    fps: u32,
    font: &'pattern_lt Font,
    // Generated text pattern owned by the rangoli module.
    rangoli_pattern: &'pattern_lt Vec<String>,     
    rangoli_pos: &'pattern_lt Vec<Vector2>,
    // Relative x- and y-offset of each English alphabet character based on the font set.
    alpha_pos: [Vector2; 26],  
}

impl<'pattern_lt> RLDriver<'pattern_lt> {
    pub fn build(rl: &'pattern_lt mut RaylibHandle, thread: &'pattern_lt RaylibThread, 
                 font: &'pattern_lt Font, rangoli_pattern: &'pattern_lt Vec<String>, 
                 rangoli_pos: &'pattern_lt Vec<Vector2>) -> RLDriver<'pattern_lt> {

        // TO-DO: Using the font set, calculate the alpha_pos values.

        RLDriver{
           rl,
           thread,
           font,
           fps: DEFAULT_FPS,
           rangoli_pattern,
           rangoli_pos,
           alpha_pos: Default::default(),
           //alpha_pos: alpha_pos,
        }
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
