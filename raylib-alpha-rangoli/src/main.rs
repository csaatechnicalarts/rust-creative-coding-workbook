#![allow(warnings)]

use clap::Parser;

#[derive(Parser)]
struct CliParam {
    #[arg(short, long)]
    number: i32,
}

static LOWER_BOUND: i32 = 1;
static UPPER_BOUND: i32 = 26;

fn print_line(n: i32, m: i32) -> String {
    let ascii_lower: [char; UPPER_BOUND as usize] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut r_line: Vec<String> = vec![];

    let mut i = n;

    while i > m {
        r_line.push(ascii_lower[i as usize].to_string());
        i -= 1;
    }

    for j in (i + 2)..(n + 1) {
        r_line.push(ascii_lower[j as usize].to_string());
    }
    r_line.join("-")
}

fn print_rangoli(n: i32) {
    let mut rangoli_lines: Vec<String> = vec![];
    for i in (-1..=(n - 2)).rev() {
        rangoli_lines.push(print_line(n - 1, i));
    }

    let max_width = rangoli_lines.last().unwrap().len();

    for line in &rangoli_lines {
        println!("{:-^width$}", line, width = max_width);
    }

    let rangoli_output = &rangoli_lines[..(rangoli_lines.len() - 1)];

    for line in rangoli_output.into_iter().rev() {
        println!("{:-^width$}", line, width = max_width);
    }
}

fn main() {
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
