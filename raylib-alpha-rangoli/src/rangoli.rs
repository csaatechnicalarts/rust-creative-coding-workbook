#![allow(warnings)]

use std::process;

pub static LOWER_BOUND: i32 = 1;
pub static UPPER_BOUND: i32 = 26;

pub fn print_rangoli(n: i32) {
    let (rangoli_output, max_width) = generate_rangoli_pattern(n);

    for line in rangoli_output.into_iter().rev() {
        println!("{:-^width$}", line, width = max_width as usize);
    }
}

pub fn generate_rangoli_pattern(n: i32) -> (Vec<String>, i32) {
    let mut rangoli_lines: Vec<String> = vec![];
    for i in (-1..=(n - 2)).rev() {
        rangoli_lines.push(print_line(n - 1, i));
    }

    let last_rline = rangoli_lines.last();
    let mut max_width = 0;

    if let Some(r_string) = last_rline {
        max_width = r_string.chars().count();
    } else {
        panic!("Error: rangoli_lines.last() yielded None!");
    }

    let rangoli_output = &rangoli_lines[..(rangoli_lines.len() - 1)];

    (rangoli_lines, max_width as i32)
}

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
