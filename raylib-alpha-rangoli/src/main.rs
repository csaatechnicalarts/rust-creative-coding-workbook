#![allow(warnings)]

use clap::Parser;

pub mod rangoli;

#[derive(Parser)]
struct CliParam {
    #[arg(short, long)]
    number: i32,
}

fn main() {
    use crate::rangoli::{print_rangoli, generate_rangoli_blob, LOWER_BOUND, UPPER_BOUND};
    use std::process;
    use raylib::prelude::*;

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

    let (rangoli_blob , max_size) = generate_rangoli_blob(cli_param.number);
    //println!("{:#?}", rangoli_blob);


    // ********************
    // Raylib set up block.
    // ********************

    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 450;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust Alphabet Rangoli (ver. 0.9)")
        .build();

    rl.set_target_fps(24);

    let font = rl.load_font(&thread, "resources/romulus.png").expect("Couldn't load font!");
    println!("\n{:#?}", font);

    let mut rangoli_pos = vec![Vector2::default(); rangoli_blob.len()];

    for i in 0..rangoli_blob.len() {
        rangoli_pos[i].x = (SCREEN_WIDTH as f32 / 2.0) - (rl.measure_text(&rangoli_blob[i], 20)/2) as f32;
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

        for i in 0..rangoli_blob.len() {
            let r_line = rangoli_blob.get(i).unwrap();
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

