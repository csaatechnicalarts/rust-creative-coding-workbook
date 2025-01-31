use crate::bubble_sort::*;

pub mod bubble_sort;

fn main() {
    let mut v = vec![2, 13, 4, 7, 8, 1, 5];

    bubble_sort(&mut v);

    assert_eq!(v, vec![1, 2, 4, 5, 7, 8, 13]);
    println!("{v:?}");
}
