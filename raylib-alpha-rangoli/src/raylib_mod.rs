#![allow(warnings)]

use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;
const DEFAULT_FPS: u32 = 24;
const FONT_SIZE: i32 = 20;

pub struct RLDriver<'pattern_lt> {
    rl: RaylibHandle,
    thread: RaylibThread,
    fps: u32,
    font: Font,
    // Generated text pattern owned by the rangoli module.
    rangoli_pattern: &'pattern_lt Vec<String>,
    // Draw call x-y coordinate for the rangoli pattern.
    rangoli_pos: Vec<Vector2>,
    // Relative x- and y-offset of each English alphabet character based on the font set.
    alpha_offset: [Vector2; 26],
}

impl<'pattern_lt> RLDriver<'pattern_lt> {
    pub fn build(
        font_file: String,
        rangoli_pattern: &'pattern_lt Vec<String>,
    ) -> RLDriver<'pattern_lt> {
        let (mut rl, thread) = raylib::init()
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("Rust Alphabet Rangoli (ver. 0.92)")
            .build();

        // Raylib-Rust calls Raylib-C using FFI. When font loading fails, the following Rust code
        // does not execute the expect() statement. Instead, the C-library handles the fault internally:
        // it prints a warning message then falls back on a default font. (See the TRACELOG message
        // in LoadBMFont() at rtext.c.) In effect, Rust doesn't panic and abort on account of
        // a failure to load fonts.

        let font = rl
            .load_font(&thread, font_file.as_str())
            .expect("Error loading font data.");

        // TO-DO: Using the font set, calculate the alpha_offset values.

        let mut rangoli_pos = vec![Vector2::default(); rangoli_pattern.len()];
        for i in 0..rangoli_pattern.len() {
            rangoli_pos[i].x = (SCREEN_WIDTH as f32 / 2.0)
                - (rl.measure_text(&rangoli_pattern[i], FONT_SIZE) / 2) as f32;
            rangoli_pos[i].y = 60.0 + (15.0 * (i as f32));
        }

        RLDriver {
            rl,
            thread,
            font,
            fps: DEFAULT_FPS,
            rangoli_pattern,
            alpha_offset: Default::default(),
            rangoli_pos: rangoli_pos,
        }
    }

    fn calc_alpha_offset() {}

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
                    FONT_SIZE as f32,
                    1.0,
                    Color::GRAY,
                );
            }
        }
    }
}
