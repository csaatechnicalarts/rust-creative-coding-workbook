use crate::bubble_sort::{proto_bubble_sort, BubbleSort, BubbleSortError};

pub mod bubble_sort;

fn main() {
    let mut v = vec![2, 13, 4, 7, 8, 1, 5];
    let mut w = v.clone();
    println!("Unsorted vector\t\t{:?}", v);
    loop {
        if bubble_sort.algo_next() == true {
            println!("Step-wise bubble sort\t{:?}", bubble_sort.get_vec());

            break;
        }
    }

    proto_bubble_sort(&mut v);
    assert_eq!(v, vec![1, 2, 4, 5, 7, 8, 13]);
    println!("Algorithmic bubble sort\t{v:?}");

    let bs = BubbleSort::new(&mut w);
    match bs {
        Ok(mut bubble_sort) => loop {
            if bubble_sort.algo_next() == true {
                println!("Step-wise bubble sort\t{:?}", bubble_sort.get_vec());

                break;
            }
        },
        Err(BubbleSortError::EmptyVecToSort) => {
            println!("Can't sort an empty vector!");
        }
    }
}
