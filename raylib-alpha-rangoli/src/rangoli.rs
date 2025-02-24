#![allow(warnings)]

use std::{fmt, process};

pub static LOWER_BOUND: i32 = 1;
pub static UPPER_BOUND: i32 = 26;

pub struct AlphabetSet {
    a_vec: Vec<char>,
    delimiter: char, 
    false_token: char
}

impl AlphabetSet {
    pub fn new(lower: char, upper: char) -> Self {
        Self {
            a_vec: (lower..=upper).into_iter().collect::<Vec<char>>(),
            delimiter: '-',
            false_token: ' '
        }
    }

    pub fn get_alphabet(&self) -> &Vec<char> {
        &self.a_vec
    }

    pub fn get_delimiter(&self) -> char {
        self.delimiter
    }

    pub fn get_false_token(&self) -> char {
        self.false_token
    }
}

pub struct RangoliTextPattern {
    rangoli_lines: Vec<String>,
    max_width: i32,
}

impl RangoliTextPattern {
    pub fn new(n: i32, alphabet: &AlphabetSet) -> Self {
        let mut rtp = RangoliTextPattern {
            rangoli_lines: vec![],
            max_width: 0,
        };

        let mut rangoli_lines = (-1..=(n - 2)).rev().map(|x| {
            Self::generate_text_line(n - 1, x, alphabet)
        }).collect::<Vec<String>>();

        // Up till this point, rangoli_lines has only the upper half of
        // the overall rangoli pattern. The lower half simply mirrors the upper part.

        let mut mirror_lines = rangoli_lines.clone().into_iter().rev().skip(1).collect::<Vec<String>>();
        rangoli_lines.append(&mut mirror_lines);
       
        rtp.rangoli_lines = rangoli_lines;
        rtp.max_width = rtp.rangoli_lines.iter().map(|x| x.len()).max().unwrap() as i32;

        rtp
    }

    pub fn get_rangoli_text(&self) -> (&Vec<String>, i32) {
        (&self.rangoli_lines, self.max_width as i32)
    }

    fn generate_text_line(n: i32, m: i32, alphabet: &AlphabetSet) -> String {

        fn get_token(x: i32, alphabet: &AlphabetSet) -> String {
            let c = alphabet.a_vec.get(x as usize);

            if let Some(token) = c {
                token.to_string()
            } else {
                alphabet.get_false_token().to_string()
            }
        }

        // For every rangoli line there is a left hand and a right hand segment  
        // of tokens to combine, with a pivotal token separating the two parts. 
        // Given an alphabet set, the starting value ((m+1) or (m+2)-1) of the
        // iterator chain is the index of this pivotal token.

        let mut r_line = ((m+1)..=n).rev().map(|x| get_token(x, &alphabet)).collect::<Vec<String>>();
        let mut r_line_append = ((m+2)..(n+1)).map(|x| get_token(x, &alphabet)).collect::<Vec<String>>();
        r_line.append(&mut r_line_append);

        r_line.join(&alphabet.get_delimiter().to_string())
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
        let alphabet_set = AlphabetSet::new('a', 'z');
        let rtp_01 = RangoliTextPattern::new(1, &alphabet_set);
        let rtp_03 = RangoliTextPattern::new(3, &alphabet_set);

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
        let alphabet_set = AlphabetSet::new('a', 'z');
        let mut rtp_01 = RangoliTextPattern::new(1, &alphabet_set);
        assert_eq!(rtp_01_data[0], rtp_01.next().unwrap());
        assert_eq!(None, rtp_01.next());

        let mut rtp_03 = RangoliTextPattern::new(3, &alphabet_set);
        assert_eq!(rtp_03_data[2], rtp_03.next().unwrap());
        assert_eq!(rtp_03_data[1], rtp_03.next().unwrap());
        assert_eq!(rtp_03_data[0], rtp_03.next().unwrap());
        assert_eq!(None, rtp_03.next());
    }

    #[test]
    fn test_iter_consume_loop() {
        let alphabet_set = AlphabetSet::new('a', 'z');
        let mut rtp_03 = RangoliTextPattern::new(3, &alphabet_set);
        let mut idx: i8 = 2;
        while idx >= 0 {
            assert_eq!(rtp_03_data[idx as usize], rtp_03.next().unwrap());

            idx = idx - 1;
        }
        assert_eq!(None, rtp_03.next());
    }

    #[test]
    fn test_iter_print_loop() {
        let alphabet_set = AlphabetSet::new('a', 'z');
        let mut rtp_03 = RangoliTextPattern::new(3, &alphabet_set);
        println!();
        for (idx, line) in rtp_03.iter().enumerate() {
            println!("{idx}: {line}");
        }
    }
}
