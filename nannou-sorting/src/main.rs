use std::process;
use crate::bubble_sort::{proto_bubble_sort, BubbleSort};

pub mod bubble_sort;

fn main() {
    let mut v = vec![2, 13, 4, 7, 8, 1, 5, 10, 11, 3];
    let mut w = v.clone();
    println!("Unsorted vector\t\t{:?}", v);

    proto_bubble_sort(&mut v);
    println!("Algorithmic bubble sort\t{v:?}");

    let mut bubble_sort: BubbleSort<u32>;
    let bs = BubbleSort::new(&mut w);

    match bs {
        Ok(bubble_sort_struct) => {
            bubble_sort = bubble_sort_struct;
        },
        Err(err) => {
            println!("[Error]: {} Good-bye!", err);
            process::exit(1);
        }

    }

    loop {
        bubble_sort.algo_next();
        if bubble_sort.is_sorted() == true {
            println!("Step-wise bubble sort\t{:?}", bubble_sort.get_vec());

            break;
        }
    }
}
