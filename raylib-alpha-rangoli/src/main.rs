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
    use crate::raylib_mod::RLDriver;
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

    // ****************************
    // Raylib set up and call block.
    // ****************************
    let mut rld = RLDriver::build(String::from("resources/alpha_beta.png"), &rangoli_pattern);
    rld.run();
}

