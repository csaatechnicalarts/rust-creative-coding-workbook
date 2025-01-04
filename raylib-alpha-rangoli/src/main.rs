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
    use crate::rangoli::{generate_rangoli_pattern, print_rangoli, LOWER_BOUND, UPPER_BOUND};
    use crate::raylib_mod::{RLDriver, DEFAULT_FPS, SCREEN_HEIGHT, SCREEN_WIDTH};
    use raylib::prelude::*;
    use std::process;

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

    let (rangoli_pattern, max_size) = generate_rangoli_pattern(cli_param.number);
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

    let font = rl
        .load_font(&thread, "resources/bad_alpha_beta.png")
        .expect("Couldn't load font!");
    //println!("\n{:#?}", font);

    let mut rld = RLDriver::build(&mut rl, &thread, &font, &rangoli_pattern);
    rld.run();
}
