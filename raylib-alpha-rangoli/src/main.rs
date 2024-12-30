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

    let cli_param = CliParam::parse();
    if (cli_param.number < LOWER_BOUND) || (cli_param.number > UPPER_BOUND) {
        println!(
            "Parameter NUMBER should be between {} and {}\nNUMBER: {}",
            LOWER_BOUND, UPPER_BOUND, cli_param.number
        );
        process::exit(1);
    }

    print_rangoli(cli_param.number);
}
