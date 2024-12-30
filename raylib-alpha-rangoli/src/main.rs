#![allow(warnings)]

use clap::Parser;

#[derive(Parser)]
struct CliParam {
    #[arg(short, long)]
    number: i32,
}

fn main() {
    use raylib_alpha_rangoli::{print_rangoli, LOWER_BOUND, UPPER_BOUND};
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

    print_rangoli(cli_param.number);

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

    while !rl.window_should_close() {

        // *******************
        // Raylib logic block.
        // *******************

        // ******************
        // Raylib draw block.
        // ******************

    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::GAINSBORO);
}

}
