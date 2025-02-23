#![allow(warnings)]

use clap::Parser;

pub mod rangoli;
pub mod raylib_mod;

#[derive(Parser)]
struct CliParam {
    #[arg(short, long)]
    number: i32,
    #[arg(short, long)]
    first_char: char,
    #[arg(short, long)]
    last_char: char,
}

fn main() {
    use crate::rangoli::{AlphabetSet, RangoliTextPattern, LOWER_BOUND, UPPER_BOUND};
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

    let alphabet_range_len = cli_param.last_char as i32 - cli_param.first_char as i32;
    if alphabet_range_len !=  UPPER_BOUND - 1 {
        println!("There must be exactly {} sequential characters in the alphabet range.", UPPER_BOUND);
        println!("For example '-fa -lz' or '-fA -lZ'");
        process::exit(2);
    }

    let alphabet_set = AlphabetSet::new(cli_param.first_char, cli_param.last_char);
    let mut rangoli_text_pattern = RangoliTextPattern::new(cli_param.number, &alphabet_set);

    // ********************
    // Raylib set up block.
    // ********************

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust Alphabet Rangoli (ver. 0.92)")
        .build();

    rl.set_target_fps(DEFAULT_FPS);

    // Raylib-Rust calls Raylib-C using FFI. When font loading fails, the following Rust code
    // does not print the expect() message then panic. Instead, internally the C-library falls
    // back on its default font, after printing a warning message. See rtext.c LoadBMFont()
    // TRACELOG message.

    let font = rl
        .load_font(&thread, "resources/bogus_font.png")
        .expect("Couldn't load font!");

    let mut rld = RLDriver::build(&mut rl, &thread, &font, &mut rangoli_text_pattern, &alphabet_set);
    rld.run();
}
