#![allow(warnings)]

use std::{fmt, path::Display, process};

pub static LOWER_BOUND: i32 = 1;
pub static UPPER_BOUND: i32 = 26;

pub struct RangoliTextPattern {
    rangoli_lines: Vec<String>,
    max_width: i32,
}

impl RangoliTextPattern {
    pub fn new(n: i32) -> Self {
        let mut rtp = RangoliTextPattern {
            rangoli_lines: vec![],
            max_width: 0,
        };

        for i in (-1..=(n - 2)).rev() {
            rtp.rangoli_lines.push(Self::generate_text_line(n - 1, i));
        }

        let last_rline = rtp.rangoli_lines.last();

        if let Some(r_string) = last_rline {
            rtp.max_width = r_string.chars().count() as i32;
        } else {
            panic!("Error: rangoli_lines.last() yielded None!");
        }

        rtp
    }

    pub fn get_rangoli_text(&self) -> (&Vec<String>, i32) {
        (&self.rangoli_lines, self.max_width as i32)
    }

    fn generate_text_line(n: i32, m: i32) -> String {
        let ascii_lower: [char; UPPER_BOUND as usize] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
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
}

impl fmt::Display for RangoliTextPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\nRangoli Text Pattern:\n{:#?}\nMax Width: {}\n",
            self.rangoli_lines, self.max_width
        )
    }
}
