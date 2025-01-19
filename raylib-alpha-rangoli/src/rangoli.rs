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
}

impl Iterator for RangoliTextPattern {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.rangoli_lines.pop() {
            Some(String) => Some(String),
            None => None,
        }
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

    #[test]
    fn test_vec_string() {
        let rtp_01 = RangoliTextPattern::new(1);
        let rtp_03 = RangoliTextPattern::new(3);

        let (rtp_01_vec, _) = rtp_01.get_rangoli_text();
        assert_eq!(*rtp_01_vec.get(0).unwrap(), String::from("a"));
        assert_ne!(*rtp_01_vec.get(0).unwrap(), String::from("z"));

        let (rtp_03_vec, _) = rtp_03.get_rangoli_text();
        assert_eq!("c", rtp_03_vec.get(0).unwrap());
        assert_eq!("c-b-c", rtp_03_vec.get(1).unwrap());
        assert_eq!("c-b-a-b-c", rtp_03_vec.get(2).unwrap());
        assert_eq!(None, rtp_03_vec.get(3));
    }

    #[test]
    fn test_iter_consume() {
        let mut rtp_01 = RangoliTextPattern::new(1);
        assert_eq!("a", rtp_01.next().unwrap());
        assert_eq!(None, rtp_01.next());

        let mut rtp_03 = RangoliTextPattern::new(3);
        assert_eq!("c-b-a-b-c", rtp_03.next().unwrap());
        assert_eq!("c-b-c", rtp_03.next().unwrap());
        assert_eq!("c", rtp_03.next().unwrap());
        assert_eq!(None, rtp_03.next());
    }
}
