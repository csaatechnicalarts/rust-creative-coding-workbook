//! **crate:nannou_sorting::BubbleSort** implements the bubble sort algorithm in two versions:
//!
//! - A textbook version of algorithm.
//! - A step-wise implementation with step-forward and step-backward functionalities.
//!
//! Combined with GUI functionality provided by [Nannou,](https://github.com/nannou-org/nannou) we can graphically walk through
//! (or walk back) the steps of the bubble sort algorithm.

use std::fmt;
use std::fmt::Debug;
use std::collections::BTreeMap;

#[derive(PartialEq, Debug)]
pub enum AlgoPrevAction<T> {
    UndoSwap { 
        outer_idx: u32, 
        inner_idx: u32,
        i_val: T, 
        ipp_val: T
    },
    NoSwap {
        outer_idx: u32,
        inner_idx: u32
    },
    EmptySwapEvents()
}

impl<T: std::fmt::Display> fmt::Display for AlgoPrevAction<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlgoPrevAction::UndoSwap {outer_idx: o_idx, inner_idx: i_idx, i_val: iv, ipp_val: ippv } 
                => write!(f, "outer_idx: {}, inner_idx: {}, ith_val: {}, inext_val: {}",
                      o_idx, i_idx, iv, ippv),
            AlgoPrevAction::NoSwap { outer_idx: o_idx, inner_idx: i_idx }
                => write!(f, "outer_idx: {}, inner_idx: {}", o_idx, i_idx),
            AlgoPrevAction::EmptySwapEvents() => write!(f, "No swap events recorded.")
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum AlgoNextAction<T> {
    Swap {
        outer_idx: u32, 
        inner_idx: u32, 
        i_val: T, 
        ipp_val: T
    },
    NoSwap {
        outer_idx: u32, 
        inner_idx: u32
    },
    BookKeeping()
}

impl<T: std::fmt::Display> fmt::Display for AlgoNextAction<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlgoNextAction::Swap { outer_idx: o_idx, inner_idx: i_idx, i_val: iv, ipp_val: ippv } 
                => write!(f, "outer_idx: {}, inner_idx: {}, i_val: {}, ipp_val: {}", 
                          o_idx, i_idx, iv, ippv),
            AlgoNextAction::NoSwap { outer_idx: o_idx, inner_idx: i_idx}
                => write!(f, "outer_idx: {}, inner_idx: {}", o_idx, i_idx),
            AlgoNextAction::BookKeeping() => write!(f, "Only book-keeping operations were performed.")
        }
    }
}

/// This type encapsulates the stream of data to sort. In the textbook version of the bubble sort algorithm, two nested loops drive the sorting process forward. For the step-wise implementation here, the BubbleSort type also extracts the two loop indices for keeping track of them globally.
///
/// BubbleSort::swap_events collects swap actions at particular points in the bubble sort process. The components of the BTreeMap are as follows:
///
/// - Key - (u32, u32) where the first integer is the index of the bubble sort outer loop (self.outer_idx) and the second one is the inner index (self.inner_idx).
/// - Value - Option::Some(T, T) stores the two values prior to being swapped. Option::None means no swap occurred.

#[derive(Debug, PartialEq)]
pub struct BubbleSort<'a, T>
where
    T: PartialOrd + Debug + Clone,
{
    v: &'a mut Vec<T>,
    v_len: u32,
    outer_idx: u32,
    inner_idx: u32,
    sort_complete: bool,
    swap_events: BTreeMap<(u32, u32), Option<(T, T)>>,
}

impl<'a, T> BubbleSort<'a, T>
where
    T: PartialOrd + Debug + Clone + std::fmt::Display,
{
    /// The BubbleSort constructor method.
    ///
    /// # Example Usage:
    ///
    /// ```
    /// use std::process;
    /// ...
    ///
    /// let mut bubble_sort = BubbleSort::new(&mut w).unwrap_or_else(|err| {
    ///   println!("[Error]: {} Good-bye!", err);
    ///   process::exit(1);
    /// });
    ///
    /// loop {
    ///   bubble_sort.algo_next();
    ///   if bubble_sort.is_sorted() == true {
    ///     println!("Step-wise bubble sort\t{:?}", bubble_sort.get_vec());
    ///
    ///     break;
    ///   }
    /// }
    /// ```

    pub fn new(v: &'a mut Vec<T>) -> Result<Self, &'static str> {
        if v.len() > 0 {
            let v_len = v.len() as u32;
            let bubble_sort = BubbleSort {
                v,
                v_len,
                outer_idx: 0,
                inner_idx: 0,
                sort_complete: false,
                swap_events: BTreeMap::new(),
            };
            Ok(bubble_sort)
        } else {
            Err("BubbleSort::new() needs a non-empty vector parameter.")
        }
    }

    pub fn get_vec(&'a self) -> &'a Vec<T> {
        self.v
    }

    pub fn is_sorted(&self) -> bool {
        return self.sort_complete;
    }

    pub fn original_state(&self) -> bool {
        return (self.outer_idx == 0)
            && (self.inner_idx == 0)
            && (self.sort_complete == false)
            && (self.swap_events.len() == 0);
    }

    /// A step-wise version of the bubble sort algorithm. It is the analog to BubbleSort::algo_prev().
    /// As per the algorithm, algo_next() compares the data at i-th and i-plus-one-th indices and
    /// swaps them accordingly, or not. It then adjusts global variables that track the state of
    /// the algorithm's progress. The function also updates the swap_events mapping, recording the
    /// occurrance or absence of a swap at a given (i-th, i-plus-one-th) combination.
    ///
    /// Note that calls to algo_next() do not always result in a comparison-and-swap operation.
    /// Instead algo_next() may take book-keeping steps to adjust global vaiables that track
    /// the algorithm's progress.
    ///
    /// # Example Usage
    ///
    /// ```
    /// let mut bubble_sort = BubbleSort::new(&mut v); // v is a vector of data to sort
    /// ...
    /// loop {
    ///     bubble_sort::algo_next();
    ///     if bubble_sortis_sorted() == true {
    ///         ...
    ///         break;
    ///     }
    /// }
    /// ```

    pub fn algo_next(&mut self) -> AlgoNextAction<T> {
        let ret_val: AlgoNextAction<T>;
        if !self.sort_complete && (self.outer_idx < self.v_len) {
            if self.inner_idx < (self.v_len - 1 - self.outer_idx) {
                if self.v[self.inner_idx as usize] > self.v[self.inner_idx as usize + 1] {
                    self.swap_events.insert(
                        (self.outer_idx, self.inner_idx),
                        Some((
                            self.v[self.inner_idx as usize].clone(),
                            self.v[self.inner_idx as usize + 1].clone(),
                        )),
                    );
                    self.v
                        .swap(self.inner_idx as usize, self.inner_idx as usize + 1);

                    ret_val = AlgoNextAction::Swap {
                        outer_idx: self.outer_idx, 
                        inner_idx: self.inner_idx,
                        i_val: self.v[self.inner_idx as usize].clone(),
                        ipp_val: self.v[self.inner_idx as usize + 1].clone(),
                    };
                } else {
                    // No swap occured.
                    self.swap_events
                        .insert((self.outer_idx, self.inner_idx), None);

                    ret_val = AlgoNextAction::NoSwap {
                        outer_idx: self.outer_idx, 
                        inner_idx: self.inner_idx
                    };
                }
                self.inner_idx = self.inner_idx + 1;
            } else {
                self.inner_idx = 0;
                self.outer_idx = self.outer_idx + 1;

                ret_val = AlgoNextAction::BookKeeping();
            }
        } else {
            self.sort_complete = true;

            ret_val = AlgoNextAction::BookKeeping();
        }

        ret_val
    }

    /// A step-wise reversal of the bubble sort algorithm. This function is the analog to BubbleSort::algo_next().
    /// Calls to algo_next() update the swap_events map, archiving the presence or absense of data swaps,
    /// Option::Some(u32, u32) or Option::None respectively. A corresponding call to algo_prev() reverts the data
    /// stream and the global variables to their prior state, clearing the last entry to the swap_events map accordingly.
    /// This function ignores book-keeping steps taken in previous algo_next() calls.

    pub fn algo_prev(&mut self) -> AlgoPrevAction<T> {
        let ret_val: AlgoPrevAction<T>;
        let last_swap_event = self.swap_events.last_key_value();

        match last_swap_event {
            Some((key_swap_event, val_swap_event)) => {
                let swap_outer_idx = key_swap_event.0;
                let swap_inner_idx = key_swap_event.1;

                match val_swap_event {
                    Some((ref i_val, ref ipp_val)) => {
                        // i_val is the archived value at inner_idx.
                        // ipp_val is the archived value at "i plus-plus", i.e. inner_idx + 1.
                        self.v[swap_inner_idx as usize] = i_val.clone();
                        self.v[swap_inner_idx as usize + 1] = ipp_val.clone();

                        /*ret_val = AlgoPrevAction::UndoSwap(
                            (swap_outer_idx, swap_inner_idx),
                            (i_val.clone(), ipp_val.clone()),
                        );*/
                    
                        ret_val = AlgoPrevAction::UndoSwap {
                            outer_idx: swap_outer_idx, 
                            inner_idx: swap_inner_idx,
                            i_val: i_val.clone(), 
                            ipp_val: ipp_val.clone()
                        }
}
                    None => {
                        // No exchange between i_val and ipp_val had transpired. Do nothing.
                        //ret_val = AlgoPrevAction::NoSwap((swap_outer_idx, swap_inner_idx));
                        ret_val = AlgoPrevAction::NoSwap {
                            outer_idx: swap_outer_idx, 
                            inner_idx: swap_inner_idx
                        };
                    }
                }
                // Whether there was a swap recorded or not, reverse the progress of the algorithm.
                self.swap_events
                    .remove(&(swap_outer_idx as u32, swap_inner_idx as u32));

                self.inner_idx = swap_inner_idx;
                self.outer_idx = swap_outer_idx;

                if self.sort_complete == true {
                    self.sort_complete = false;
                }
            }
            None => {
                // Empty swap_events map. Nothing to undo.
                ret_val = AlgoPrevAction::EmptySwapEvents();
            }
        }

        ret_val
    }
}

/// Textbook implementation of bubble sort, mainly for reference.
pub fn canonical_bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i + 1] {
                // std::mem::swap() - Swaps the value at two mutable locations,
                // without deinitalizing either one.
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_canonical_bubble_sort() {
        let mut v = vec![2, 13, 4, 7, 8, 1, 5];
        let mut w = v.clone();
        canonical_bubble_sort(&mut w);
        v.sort();
        assert_eq!(w, v);
    }

    
    #[test]
    fn test_canonical_bubble_sort_contra() {
        let mut a = vec!['a', 'x', 'm', 'n', 'h', 'c'];
        canonical_bubble_sort(&mut a);
        assert_ne!(a, vec!['a']);
    }

    #[test]
    fn test_primitive_types() {
        let mut v_int = vec![2, 3, 1];
        let mut v_float = vec![2.2, 3.3, 1.1];
        let mut v_char = vec!['2', '3', '1'];
        let mut v_str = vec!["2", "3", "1"];
        let mut v_bool = vec![false, true, false, true];
    
        let mut bs_int_type: BubbleSort<u32> = BubbleSort::new(&mut v_int).unwrap();
        loop {
            bs_int_type.algo_next();
            if bs_int_type.is_sorted() == true {
                println!("Step-wise bubble sort\t{:?}", bs_int_type.get_vec());

                break;
            }
        }
        assert_eq!(v_int, vec![1, 2, 3]);
 
        let mut bs_float_type: BubbleSort<f32> = BubbleSort::new(&mut v_float).unwrap();
        loop {
            bs_float_type.algo_next();
            if bs_float_type.is_sorted() == true {
                println!("Step-wise bubble sort\t{:?}", bs_float_type.get_vec());

                break;
            }
        }
        assert_eq!(v_float, vec![1.1, 2.2, 3.3]);
 
        let mut bs_char_type: BubbleSort<char> = BubbleSort::new(&mut v_char).unwrap();
        loop {
            bs_char_type.algo_next();
            if bs_char_type.is_sorted() == true {
                println!("Step-wise bubble sort\t{:?}", bs_char_type.get_vec());

                break;
            }
        }
        assert_eq!(v_char, vec!['1', '2', '3']);
 
        let mut bs_str_type: BubbleSort<&str> = BubbleSort::new(&mut v_str).unwrap();
        loop {
            bs_str_type.algo_next();
            if bs_str_type.is_sorted() == true {
                println!("Step-wise bubble sort\t{:?}", bs_str_type.get_vec());

                break;
            }
        }
        assert_eq!(v_str, vec!["1", "2", "3"]);
 
        let mut bs_bool_type: BubbleSort<bool> = BubbleSort::new(&mut v_bool).unwrap();
        loop {
            bs_bool_type.algo_next();
            if bs_bool_type.is_sorted() == true {
                println!("Step-wise bubble sort\t{:?}", bs_bool_type.get_vec());

                break;
            }
        }
        // false -> 0 and true -> 1; sorted accordingly.
        assert_eq!(v_bool, vec![false, false, true, true]);
    }

    
    #[test]
    fn test_proto_vs_stepwise_sorting() {
        let mut v = vec![2, 13, 4, 7, 8, 1, 5, 10, 11, 3];
        let mut w = v.clone();
        println!("Unsorted vector\t\t{:?}", v);

        canonical_bubble_sort(&mut w);
        println!("Algorithmic bubble sort\t{w:?}");

        // Safe to assume Result::Ok is returned, given that bubble_sort takes in
        // a mutable reference to a non-empty vector v.
        let mut bubble_sort: BubbleSort<u32> = BubbleSort::new(&mut v).unwrap();
        loop {
            bubble_sort.algo_next();
            if bubble_sort.is_sorted() == true {
                println!("Step-wise bubble sort\t{:?}", bubble_sort.get_vec());

                break;
            }
        }

        // Duplicate asserts really, as bubble_sort owns the mutable reference to vector v.
        assert_eq!(*bubble_sort.get_vec(), w);
        assert_eq!(v, w);
    }

    
    #[test]
    fn test_empty_input() {
        let mut v0: Vec<u32> = Vec::new();
        let bs0 = BubbleSort::new(&mut v0);
        assert_eq!(
            bs0,
            Err("BubbleSort::new() needs a non-empty vector parameter.")
        );

        let mut default_vec = vec![3, 2, 1];
        let default_vec_len = default_vec.len() as u32;
        let mut bs1 = BubbleSort::new(&mut v0).unwrap_or_else(|err| {
            println!("\n[Err]: {}\nUsing default vector instead.\n", err);

            BubbleSort {
                v: &mut default_vec,
                v_len: default_vec_len,
                outer_idx: 0,
                inner_idx: 0,
                sort_complete: false,
                swap_events: BTreeMap::new(),
            }
        });

        loop {
            bs1.algo_next();
            if bs1.is_sorted() {
                assert_eq!(*bs1.get_vec(), vec![1, 2, 3]);

                break;
            }
        }
    }

    
    #[test]
    fn test_single_element() {
        let mut v = vec![1];
        let bs = BubbleSort::new(&mut v);
        match bs {
            Ok(mut bubble_sort) => {
                println!("{:?}", bubble_sort);
                bubble_sort.algo_next();
                println!("{:?}", bubble_sort);
                bubble_sort.algo_next();
                assert_eq!(bubble_sort.v_len, 1);
                assert_eq!(bubble_sort.is_sorted(), true);
                println!("{:?}", bubble_sort);
            }
            Err(msg) => {
                println!("{msg}");
            }
        }
    }

    
    #[test]
    fn test_bubble_sort_constructor() {
        let mut v1 = vec![4, 2, 1];
        let bs1 = BubbleSort::new(&mut v1);
        assert_ne!(
            bs1,
            Err("BubbleSort::new() needs a non-empty vector parameter.")
        );

        let bs = bs1.unwrap();
        assert_eq!(bs.v, &vec![4, 2, 1]);
        assert_ne!(bs.v.get(0), Some(&u32::MAX));
        assert_eq!(bs.v.get(0), Some(&4));
        assert_eq!(bs.v.get(1), Some(&2));
        assert_eq!(bs.v.get(2), Some(&1));
        assert_eq!(bs.v.get(3), None);

        assert_eq!(bs.v_len, 3);
        assert_eq!(bs.outer_idx, 0);
        assert_eq!(bs.inner_idx, 0);
    }

    
    #[test]
    fn test_algo_prev_basic() {
        let mut v = vec![2, 1];
        let mut bs = BubbleSort::new(&mut v);
        match bs {
            Ok(ref mut bubble_sort) => {
                println!("\n[0] {:?}", bubble_sort);
                assert_eq!(bubble_sort.original_state(), true);

                bubble_sort.algo_next();
                println!("[1] {:?}", bubble_sort);
                assert_eq!(bubble_sort.original_state(), false);

                bubble_sort.algo_prev();
                println!("[1-2] {:?}", bubble_sort);
                assert_eq!(bubble_sort.original_state(), true);
                assert_eq!(*bubble_sort.get_vec(), vec![2, 1]);

                bubble_sort.algo_prev();
                println!("[1-2] {:?}", bubble_sort);
                assert_eq!(bubble_sort.original_state(), true);
                assert_eq!(*bubble_sort.get_vec(), vec![2, 1]);

                bubble_sort.algo_next();
                println!("[2] {:?}", bubble_sort);
                assert_eq!(bubble_sort.is_sorted(), false);
                assert_eq!(bubble_sort.swap_events.len(), 1);
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2]);

                bubble_sort.algo_next();
                println!("[3] {:?}", bubble_sort);
                assert_eq!(bubble_sort.is_sorted(), false);
                assert_eq!(bubble_sort.original_state(), false);
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2]);

                bubble_sort.algo_prev();
                println!("[3-4] {:?}", bubble_sort);
                assert_eq!(bubble_sort.is_sorted(), false);
                assert_eq!(bubble_sort.original_state(), true);
                assert_eq!(*bubble_sort.get_vec(), vec![2, 1]);

                bubble_sort.algo_next();
                println!("[4] {:?}", bubble_sort);
                assert_eq!(bubble_sort.is_sorted(), false);
                assert_eq!(bubble_sort.original_state(), false);
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2]);

                bubble_sort.algo_prev();
                println!("[4-5] {:?}", bubble_sort);
                assert_eq!(bubble_sort.is_sorted(), false);
                assert_eq!(bubble_sort.original_state(), true);
                assert_eq!(*bubble_sort.get_vec(), vec![2, 1]);

                bubble_sort.algo_next();
                println!("[5] {:?}", bubble_sort);
                assert_eq!(bubble_sort.is_sorted(), false);
                assert_eq!(bubble_sort.original_state(), false);
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2]);

                bubble_sort.algo_next();
                println!("[6] {:?}", bubble_sort);
                assert_eq!(bubble_sort.is_sorted(), false);
                assert_eq!(bubble_sort.original_state(), false);
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2]);

                bubble_sort.algo_next();
                println!("[7] {:?}", bubble_sort);
                assert_eq!(bubble_sort.is_sorted(), false);
                assert_eq!(bubble_sort.original_state(), false);
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2]);

                bubble_sort.algo_next();
                println!("[8] {:?}", bubble_sort);
                assert_eq!(bubble_sort.is_sorted(), true);
                assert_eq!(bubble_sort.original_state(), false);
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2]);
            }
            Err(msg) => {
                println!("{msg}");
            }
        }
    }

    
    #[test]
    fn test_algo_prev_loops() {
        let u = vec![1, 3, 2, 7, 12, 8, 6, 5, 11, 4, 9, 10];
        let mut v = u.clone();
        let bs = BubbleSort::new(&mut v);
        match bs {
            Ok(mut bubble_sort) => {
                println!("[0] {:?}", bubble_sort);
                loop {
                    bubble_sort.algo_next();
                    if bubble_sort.is_sorted() == true {
                        let mut w = u.clone();
                        w.sort();
                        assert_eq!(*bubble_sort.get_vec(), w);
                        assert_eq!(bubble_sort.original_state(), false);

                        break;
                    }
                }

                println!("[1] {:?}", bubble_sort);
                loop {
                    if bubble_sort.original_state() {
                        println!("[2] {:?}", bubble_sort);
                        assert_eq!(*bubble_sort.get_vec(), u);
                        assert_eq!(bubble_sort.original_state(), true);

                        break;
                    } else {
                        bubble_sort.algo_prev();
                    }
                }
            }
            Err(msg) => {
                println!("{msg}");
            }
        }
    }

    #[test]
    fn test_algo_prev_sorted_vec() {
        let mut u = vec![1, 2, 3];
        let bs_01 = BubbleSort::new(&mut u);
        if let Ok(mut bubble_sort) = bs_01 {
            bubble_sort.algo_prev();
            assert_eq!(bubble_sort.original_state(), true);
            assert_eq!(bubble_sort.is_sorted(), false);

            while bubble_sort.is_sorted() == false {
                bubble_sort.algo_next();
            }
            println!("{:?}", bubble_sort);
            assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 3]);
            assert_eq!(bubble_sort.original_state(), false);

            let prev_action = bubble_sort.algo_prev();
            println!("{:?}", bubble_sort);

            assert_eq!(prev_action, AlgoPrevAction::NoSwap { outer_idx: 1, inner_idx: 0 });
            println!("{:#?}", prev_action);
            assert_eq!(bubble_sort.is_sorted(), false);
            assert_eq!(bubble_sort.original_state(), false);

            let prev_action = bubble_sort.algo_prev();
            println!("{:?}", bubble_sort);

            assert_eq!(prev_action, AlgoPrevAction::NoSwap { outer_idx: 0, inner_idx: 1 });
            println!("{:#?}", prev_action);
            assert_eq!(bubble_sort.is_sorted(), false);
            assert_eq!(bubble_sort.original_state(), false);

            let prev_action = bubble_sort.algo_prev();
            println!("{:?}", bubble_sort);

            assert_eq!(prev_action, AlgoPrevAction::NoSwap { outer_idx: 0, inner_idx: 0 });
            println!("{:#?}", prev_action);
            assert_eq!(bubble_sort.is_sorted(), false);
            assert_eq!(bubble_sort.original_state(), true);
        }
    }

    
    #[test]
    fn test_bubble_sort_step() {
        let mut v = vec![4, 1, 2];
        let bs = BubbleSort::new(&mut v);
        match bs {
            Ok(mut bubble_sort) => {
                println!("\nUnsorted vector\t\t{:?}", bubble_sort.get_vec());
                //println!("{:#?}", bubble_sort);

                bubble_sort.algo_next();
                assert_eq!(*bubble_sort.get_vec(), vec![1, 4, 2]);
                assert_eq!(bubble_sort.swap_events.len(), 1);
                assert_eq!(
                    *bubble_sort.swap_events.get(&(0 as u32, 0 as u32)).unwrap(),
                    Some((4, 1))
                );
                println!("{:?}", bubble_sort);

                bubble_sort.algo_next();
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 4]);
                assert_eq!(bubble_sort.swap_events.len(), 2);
                assert_eq!(
                    *bubble_sort.swap_events.get(&(0 as u32, 1 as u32)).unwrap(),
                    Some((4, 2))
                );
                println!("{:?}", bubble_sort);

                // In this next function call the bubble sort algorithm is paused. Only
                // housekeeping tasks are done here, adjusting the inner and outer indices.

                bubble_sort.algo_next();
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 4]);
                assert_eq!(bubble_sort.swap_events.len(), 2);
                println!("{:?}", bubble_sort);

                // Here the bubble sort algorithm resumes. The third and final swap takes place.

                bubble_sort.algo_next();
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 4]);
                assert_eq!(bubble_sort.swap_events.len(), 3);
                assert_eq!(
                    *bubble_sort.swap_events.get(&(1 as u32, 0 as u32)).unwrap(),
                    None
                );
                println!("{:?}", bubble_sort);

                bubble_sort.algo_next();
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 4]);
                assert_eq!(bubble_sort.swap_events.len(), 3);
                println!("{:?}", bubble_sort);

                bubble_sort.algo_next();
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 4]);
                assert_eq!(bubble_sort.swap_events.len(), 3);
                println!("{:?}", bubble_sort);

                bubble_sort.algo_next();
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 4]);
                assert_eq!(bubble_sort.swap_events.len(), 3);
                println!("{:?}", bubble_sort);

                assert_eq!(bubble_sort.is_sorted(), true);

                println!("Step-wise bubble sort\t{:?}\n", bubble_sort.get_vec());
                //println!("{:#?}", bubble_sort);
            }
            Err(msg) => {
                println!("{msg}");
            }
        }
    }

    
    #[test]
    fn test_pre_sorted_input_01() {
        let mut v = vec![1, 2, 3];
        let bs = BubbleSort::new(&mut v);
        match bs {
            Ok(mut bubble_sort) => {
                println!("\nPre-sorted 01 vector\t\t{:?}", bubble_sort.get_vec());
                loop {
                    bubble_sort.algo_next();
                    if bubble_sort.is_sorted() == true {
                        assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 3]);
                        assert_eq!(bubble_sort.swap_events.len(), 3);
                        assert_eq!(
                            *bubble_sort.swap_events.get(&(0 as u32, 0 as u32)).unwrap(),
                            None
                        );
                        assert_eq!(
                            *bubble_sort.swap_events.get(&(0 as u32, 1 as u32)).unwrap(),
                            None
                        );
                        assert_eq!(
                            *bubble_sort.swap_events.get(&(1 as u32, 0 as u32)).unwrap(),
                            None
                        );
                        println!("Step-wise bubble sort\t{:?}\n", bubble_sort.get_vec());

                        break;
                    }
                }
            }
            Err(msg) => {
                println!("{msg}");
            }
        }
    }

    
    #[test]
    fn test_pre_sorted_input_02() {
        let mut v = vec![1, 1, 1];
        let bs = BubbleSort::new(&mut v);
        match bs {
            Ok(mut bubble_sort) => {
                println!("\nPre-sorted 02 vector\t\t{:?}", bubble_sort.get_vec());
                loop {
                    bubble_sort.algo_next();
                    if bubble_sort.is_sorted() == true {
                        assert_eq!(*bubble_sort.get_vec(), vec![1, 1, 1]);
                        assert_eq!(bubble_sort.swap_events.len(), 3);
                        assert_eq!(
                            *bubble_sort.swap_events.get(&(0 as u32, 0 as u32)).unwrap(),
                            None
                        );
                        assert_eq!(
                            *bubble_sort.swap_events.get(&(0 as u32, 1 as u32)).unwrap(),
                            None
                        );
                        assert_eq!(
                            *bubble_sort.swap_events.get(&(1 as u32, 0 as u32)).unwrap(),
                            None
                        );
                        println!("Step-wise bubble sort\t{:?}\n", bubble_sort.get_vec());

                        break;
                    }
                }
            }
            Err(msg) => {
                println!("{msg}");
            }
        }
    }

    
    #[test]
    fn test_reverse_sorted_input() {
        let mut v = vec![3, 2, 1];
        let bs = BubbleSort::new(&mut v);
        match bs {
            Ok(mut bubble_sort) => {
                println!("\nReverse sorted vector\t\t{:?}", bubble_sort.get_vec());
                loop {
                    bubble_sort.algo_next();
                    if bubble_sort.is_sorted() == true {
                        assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 3]);
                        assert_eq!(bubble_sort.swap_events.len(), 3);
                        assert_eq!(
                            *bubble_sort.swap_events.get(&(0 as u32, 0 as u32)).unwrap(),
                            Some((3, 2))
                        );
                        assert_eq!(
                            *bubble_sort.swap_events.get(&(0 as u32, 1 as u32)).unwrap(),
                            Some((3, 1))
                        );
                        assert_eq!(
                            *bubble_sort.swap_events.get(&(1 as u32, 0 as u32)).unwrap(),
                            Some((2, 1))
                        );
                        println!("Step-wise bubble sort\t{:?}\n", bubble_sort);

                        break;
                    }
                }
            }
            Err(msg) => {
                println!("{msg}");
            }
        }
    }
}
