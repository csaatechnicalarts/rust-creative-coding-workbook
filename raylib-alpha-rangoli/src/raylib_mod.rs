#![allow(warnings)]

use raylib::prelude::*;

pub const SCREEN_WIDTH: i32 = 800;
pub const SCREEN_HEIGHT: i32 = 450;
pub const DEFAULT_FPS: u32 = 24;

pub struct RLDriver<'pattern_lt> {
    rl: RaylibHandle,
    thread: RaylibThread,
    fps: u32,
    font: Font,
    // Generated text pattern owned by the rangoli module.
    rangoli_pattern: &'pattern_lt Vec<String>,     
    // Relative x- and y-offset of each English alphabet character based on the font set.
    alpha_pos: [Vector2; 26],  
}

impl<'pattern_lt> RLDriver<'pattern_lt> {
    pub fn build(&mut self, font_file: String, rangoli_pattern: &'pattern_lt Vec<String>) -> RLDriver<'pattern_lt> {
        let (mut rl, thread) = raylib::init()
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Rust Alphabet Rangoli (ver. 0.9)")
            .build();

        // Raylib-Rust calls Raylib-C using FFI. When font loading fails, the following Rust code
        // does not execute the expect() statement. Instead, the C-library handles the fault internally: 
        // it prints a warning message then falls back on a default font. (See the TRACELOG message 
        // in LoadBMFont() at rtext.c.) In effect, Rust doesn't panic and abort on account of 
        // a failure to load fonts.

        let font = rl.load_font(&thread, font_file.as_str()).expect("Error loading font data.");

        // TO-DO: Using the font set, calculate the alpha_pos values.

        RLDriver{
           rl,
           thread,
           font,
           fps: DEFAULT_FPS,
           rangoli_pattern,
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
                    self.alpha_pos[i as usize],
                    20.0, 
                    1.0, 
                    Color::GRAY
                );
            }
        }    
    }
}
