use delegate::delegate;
use std::ops::Index;
use std::ops::IndexMut;
fn main() {
    
    // x/(log(x))*(1+3/(2log(x))) overestimate of primes whithin a range GREATER THAN 1 by Rosser and Schoenfeld
    let range:usize = 1223231214; // <----- Must be greater than 1 for this formula to work

    let leftover = 1..3;
    let mut seive: Seive = Seive::new(range);

    #[derive(Debug)]
    struct Seive {
        seive:Vec<Option<usize>>,
        current_prime_idx:usize,
        current_idx:usize,
        number_of_nones:usize
    }

    // 1. Delegate the read-only Index trait
    impl Index<usize> for Seive {
        type Output = Option<usize>;

        delegate! {
            to self.seive {
                fn index(&self, index: usize) -> &Self::Output;
            }
        }
    }

    // 2. Delegate the mutable IndexMut trait
    impl IndexMut<usize> for Seive {
        delegate! {
            to self.seive {
                fn index_mut(&mut self, index: usize) -> &mut Self::Output;
            }
        }
    }

    impl Seive {
        fn new(range:usize) -> Self {
            Self {
                seive: (3..range).step_by(2).map(|n| Some(n as usize)).collect(),
                current_prime_idx: 0,
                current_idx:0,
                number_of_nones:0
            }
        }

        fn remove_all_of_multiple(&mut self,multiple:usize, start_idx:usize) {
            let mut pstart_idx=start_idx;
            if self[start_idx].unwrap_or(start_idx * 2 + 3)%multiple!=0 {
                pstart_idx = self.find_first_multiple(multiple,start_idx)
            }
            (pstart_idx..self.len()).step_by(multiple).for_each(|idx|self[idx] = None);
        }

        delegate! {
            to self.seive {
                pub fn len(&self) -> usize;
            }
        }

        fn find_first_multiple(&self, multiple: usize, start_idx: usize) -> usize { 
            // 1. Unwrap the Option safely or use a default if it was already cleared
            // 2. Map the vector index back to its physical odd number value: (index * 2) + 3
            let current_val = self[start_idx].unwrap_or(start_idx * 2 + 3);
            let diff_to_last_multiple_idx = (current_val % multiple) / 2;
            start_idx + multiple - diff_to_last_multiple_idx
        }

    }


    // 1. Correct the upper bound calculation for the loop
    let limit = ((range as f64).sqrt() as usize) / 2;

    while seive.current_idx < limit {
        if let Some(prime_value) = seive[seive.current_idx] {
            seive.current_prime_idx = seive.current_idx;
            
            // 2. Map the prime value to its perfect starting index: (prime^2 - 3) / 2
            let start_idx = (prime_value * prime_value - 3) / 2;
            
            // 3. Pass the actual prime value, and the calculated index
            seive.remove_all_of_multiple(prime_value, start_idx);
        }
        seive.current_idx += 1;
    }

    println!("{:?}", seive)
}

//9Gb 1 minute 30 seconds