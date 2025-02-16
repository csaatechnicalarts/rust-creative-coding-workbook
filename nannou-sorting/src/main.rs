use std::{io, process};
use crate::bubble_sort::BubbleSort;

pub mod bubble_sort;

/// The main function for the nannou-sorting binary. The program expect a stream of
/// integers from stdin which it, in turn, sorts and prints out. In the absence of
/// data piped in the command line, as shown in the example below, nannou-sorting
/// enters interactive mode, accepting integer inputs separated by new line until
/// it reads the EOF (Ctr-D) character.
///
/// # Example Usage:
///
/// ```
/// $> shuf -i 1-100 -n 10 | ./nannou-sorting
/// Original input:		    [38, 51, 23, 46, 33, 96, 94, 17, 73, 54]
/// Step-wise bubble sort:	[17, 23, 33, 38, 46, 51, 54, 73, 94, 96]
/// ```

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input_str = String::new();
    let mut str_int: u32;
    let mut data_vec = Vec::<u32>::new();

    while io::stdin().read_line(&mut input_str)? != 0 {
        str_int = input_str.trim().parse::<u32>().unwrap_or_else(|err| {
            println!("{}: {:?}", err, input_str);

            0
        });

        data_vec.push(str_int);
        input_str.clear();
    }
    println!("Original input:\t\t{:?}", data_vec);

    let mut bubble_sort: BubbleSort<u32> = BubbleSort::new(&mut data_vec)
        .unwrap_or_else(|err| {
            println!("[Error]: {} Good-bye!", err);

            process::exit(1);
        });

    loop {
        bubble_sort.algo_next();
        if bubble_sort.is_sorted() == true {
            println!("Step-wise bubble sort:\t{:?}", bubble_sort.get_vec());

            break;
        }
    }

    Ok(())
}
