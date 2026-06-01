use std::env;
use std::ops::Range;

mod primes;
mod array_collect;
mod segmented_seive;

use crate::segmented_seive::SegmentedSeive;
use crate::primes::Primes;

fn main() {
    let args:Vec<String> = env::args().collect();
    
    // x/(log(x))*(1+3/(2log(x))) overestimate of primes whithin a range GREATER THAN 1 by Rosser and Schoenfeld
    let mut range:usize = 2000000000; // <----- Must be greater than 1 for this formula to work
    if args.len()>1 { range = args[1].parse().unwrap_or(1000000);}
}

//12.5Gb, 2 billion, 14 minutes 40 seconds

    // impl <const SIZE: usize>fmt::Debug for Seive<SIZE> {
    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
    //     f.debug_list()
    //         .entries(self.primes.iter())
    //         .finish()?;
    //     write!(f, ", current_checked_prime_idx: {}, current_idx: {} }}", 
    //     self.primes.len()-1, self.current_idx
    //     )
    // }}

    // // 1. Delegate the read-only Index trait
    // impl <const SIZE: usize> Index<usize> for Seive<SIZE> {
    //     type Output = Option<usize>;

    //     delegate! {
    //         to self.segmented_seive {
    //             fn index(&self, index: usize) -> &Self::Output;
    //         }
    //     }
    // }

    // // 2. Delegate the mutable IndexMut trait
    // impl <const SIZE: usize> IndexMut<usize> for Seive<SIZE> {
    //     delegate! {
    //         to self.segmented_seive {
    //             fn index_mut(&mut self, index: usize) -> &mut Self::Output;
    //         }
    //     }
    // }

    struct FullSeive<const SEG_SIZE: usize> {
        segmented_seive: SegmentedSeive<SEG_SIZE>,
        primes: Primes
    }

    impl<const SEG_SIZE: usize> FullSeive<SEG_SIZE> {
        fn new(range:usize) -> Self {
            Self {
                segmented_seive: SegmentedSeive::<SEG_SIZE>::new(range),
                primes: Primes::new(range)
            }
        }
        // fn new(range:usize) -> Self {
        //     const START_RANGE:usize=3;
        //     const STEP:usize=2;
        //     let seg_end_num:usize= SIZE*STEP+START_RANGE;
           
        //     let mut result = Self {
        //         segmented_seive: (START_RANGE..seg_end_num).step_by(STEP).map(|num| Some(num as usize)).collect_array().unwrap(),
        //         primes: Vec::with_capacity(Self::overestimate_num_of_primes(range)),
        //         current_idx:0,
        //         step:STEP,
        //         range,
        //         num_of_loops:0
        //     };
        //     result.primes.push(2);
        //     result
        // }

        // fn bump_sieve(&mut self) -> &[usize] {
        //     let seive_primes_index_range = self.drain_seive_to_primes_return_primes_index_range();
        //     self.segmented_seive = (self.seg_end()+self.step..=self.seg_end()+SIZE*self.step).step_by(self.step).map(|num| Some(num as usize)).collect_array().unwrap();
        //     self.num_of_loops+=1;
        //     self.current_idx=0;
        //     &self.primes[seive_primes_index_range]
        // }

        // fn drain_seive_to_primes_return_primes_index_range(&mut self) -> Range<usize> {
        //     let mut seg_somes = self.segmented_seive.into_iter().flatten().collect();
        //     let old_primes_len = self.primes.len();
        //     self.primes.append(&mut seg_somes);
        //     old_primes_len..self.primes.len()
        // }

        // fn get_all_primes_in_seg(&mut self) {
        //         // Loop by index to avoid borrowing self.primes
        //     for prime_idx in 0..self.primes.len() {
        //         let prime = self.primes[prime_idx]; // Copy the value out (no borrow held)
        //         if prime > Self::max_factor_to_check(prime) { break; }
        //         // Create a scoped block to control exactly when the iterator is dropped
        //          self.current_idx = {
        //             if let Some(mut prime_multiples_iter) = self.mut_multiple_iter(prime, self.current_idx) 
        //                 && let Some(peek) = prime_multiples_iter.peek() 
        //             {
        //                 let val = peek.unwrap();
        //                 prime_multiples_iter.into_iter().for_each(|multiple| *multiple = None);
        //                 val
        //             } else {self.current_idx}
        //         }; // prime_multiples_iter and its borrow on `self` are completely dropped HERE
        //     }
        // }
        


        // delegate! {
        //     to self.segmented_seive {
        //         pub fn len(&self) -> usize;
        //     }
        // }

    }