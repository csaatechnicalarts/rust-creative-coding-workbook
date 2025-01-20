#![allow(warnings)]

use std::{fmt, process};

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

    pub fn iter(&self) -> std::slice::Iter<'_, std::string::String> {
        self.rangoli_lines.iter()
    }
}

impl Iterator for RangoliTextPattern {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.rangoli_lines.pop()
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

#[cfg(test)]
mod tests {
    use super::*;

    const rtp_bogus: [&str; 1] = ["z"];
    const rtp_01_data: [&str; 1] = ["a"];
    const rtp_03_data: [&str; 3] = ["c", "c-b-c", "c-b-a-b-c"];

    #[test]
    fn test_vec_string() {
        let rtp_01 = RangoliTextPattern::new(1);
        let rtp_03 = RangoliTextPattern::new(3);

        let (rtp_01_vec, _) = rtp_01.get_rangoli_text();
        assert_eq!(*rtp_01_vec.get(0).unwrap(), rtp_01_data[0]);
        assert_ne!(*rtp_01_vec.get(0).unwrap(), rtp_bogus[0]);

        let (rtp_03_vec, _) = rtp_03.get_rangoli_text();
        assert_eq!(rtp_03_data[0], rtp_03_vec.get(0).unwrap());
        assert_eq!(rtp_03_data[1], rtp_03_vec.get(1).unwrap());
        assert_eq!(rtp_03_data[2], rtp_03_vec.get(2).unwrap());
        assert_eq!(None, rtp_03_vec.get(3));
    }

    #[test]
    fn test_iter_consume() {
        let mut rtp_01 = RangoliTextPattern::new(1);
        assert_eq!(rtp_01_data[0], rtp_01.next().unwrap());
        assert_eq!(None, rtp_01.next());

        let mut rtp_03 = RangoliTextPattern::new(3);
        assert_eq!(rtp_03_data[2], rtp_03.next().unwrap());
        assert_eq!(rtp_03_data[1], rtp_03.next().unwrap());
        assert_eq!(rtp_03_data[0], rtp_03.next().unwrap());
        assert_eq!(None, rtp_03.next());
    }

    #[test]
    fn test_iter_consume_loop() {
        let mut rtp_03 = RangoliTextPattern::new(3);
        let mut idx: i8 = 2;
        while idx >= 0 {
            assert_eq!(rtp_03_data[idx as usize], rtp_03.next().unwrap());

            idx = idx - 1;
        }
        assert_eq!(None, rtp_03.next());
    }

    #[test]
    fn test_iter_print_loop() {
        let mut rtp_03 = RangoliTextPattern::new(3);
        println!();
        for (idx, line) in rtp_03.iter().enumerate() {
            println!("{idx}: {line}");
        }
    }
}
