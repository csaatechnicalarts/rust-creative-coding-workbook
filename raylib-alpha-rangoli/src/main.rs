#![allow(warnings)]

use clap::Parser;

pub mod rangoli;
pub mod raylib_mod;

#[derive(Parser)]
struct CliParam {
    #[arg(short, long)]
    number: i32,
}

fn main() {
    use std::process;
    use raylib::prelude::*;
    use crate::raylib_mod::{SCREEN_HEIGHT, SCREEN_WIDTH, DEFAULT_FPS, RLDriver};
    use crate::rangoli::{print_rangoli, generate_rangoli_pattern, LOWER_BOUND, UPPER_BOUND};

    // *************************
    // Rangoli generation block.
    // *************************

    let cli_param = CliParam::parse();
    if (cli_param.number < LOWER_BOUND) || (cli_param.number > UPPER_BOUND) {
        println!(
            "Parameter NUMBER should be between {} and {}\nNUMBER: {}",
            LOWER_BOUND, UPPER_BOUND, cli_param.number
        );
        process::exit(1);
    }

    //print_rangoli(cli_param.number);

    let (rangoli_pattern , max_size) = generate_rangoli_pattern(cli_param.number);
    //println!("{:#?}", rangoli_pattern);


    // ********************
    // Raylib set up block.
    // ********************


    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust Alphabet Rangoli (ver. 0.9)")
        .build();

    rl.set_target_fps(DEFAULT_FPS);

    // Raylib-Rust calls Raylib-C using FFI. When font loading fails, the following Rust code
    // does not print the expect() message then panic. Instead, internally the C-library falls
    // back on its default font, after printing a warning message. See rtext.c LoadBMFont() 
    // TRACELOG message.
 
    let font = rl.load_font(&thread, "resources/romulus.png").expect("Couldn't load font!");
    println!("\n{:#?}", font);

    //let rld = RLDriver::build(&mut rl, &thread, DEFAULT_FPS, &font, &rangoli_pattern);

    let mut rangoli_pos = vec![Vector2::default(); rangoli_pattern.len()];

    for i in 0..rangoli_pattern.len() {
        rangoli_pos[i].x = (SCREEN_WIDTH as f32 / 2.0) - (rl.measure_text(&rangoli_pattern[i], 20)/2) as f32;
        rangoli_pos[i].y = 60.0 + (15.0 * (i as f32));
    }
    //println!("\n{:#?}", rangoli_pos);

    while !rl.window_should_close() {

        // *******************
        // Raylib logic block.
        // *******************

        // ******************
        // Raylib draw block.
        // ******************

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::GAINSBORO);

        for i in 0..rangoli_pattern.len() {
            let r_line = rangoli_pattern.get(i).unwrap();
            d.draw_text_ex(
                &font, 
                r_line, 
                rangoli_pos[i as usize],
                20.0, 
                1.0, 
                Color::GRAY
            );
        }
    }
}

