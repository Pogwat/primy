use delegate::delegate;
use std::ops::Index;
use std::ops::IndexMut;
use std::env;
use std::fmt;
fn main() {
    let args:Vec<String> = env::args().collect();
    
    // x/(log(x))*(1+3/(2log(x))) overestimate of primes whithin a range GREATER THAN 1 by Rosser and Schoenfeld
    let mut range:usize = 2000000000; // <----- Must be greater than 1 for this formula to work
    if args.len()>1 { range = args[1].parse().unwrap_or(1000000);}


    let leftover = 1..3;
    let mut seive: Seive = Seive::new(range);

    struct Seive {
        seive:Vec<Option<usize>>,
        current_checked_prime_idx:usize,
        current_idx:usize,
        start:usize, /* from 0 */ 
        spacing:usize,
        range:usize
    }

    impl fmt::Debug for Seive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        f.debug_list()
            .entries(self.seive.iter().flatten())
            .finish()?;
        write!(f, ", current_checked_prime_idx: {}, current_idx: {} }}", 
        self.current_checked_prime_idx, self.current_idx
        )
    }}

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
                seive: (3..=range).step_by(2).map(|n| Some(n as usize)).collect(),
                current_checked_prime_idx: 0,
                current_idx:0,
                start:3,
                spacing:2,
                range:range
            }
        }

        fn fast_calc_at_index(&self, idx:usize) -> usize {self.spacing*idx+self.start}

        fn checked_remove_multiple(&mut self, multiple: usize, start_idx: usize) {       
            if start_idx < self.len() { //Dosent go out of bounds even if start_idx+multiple>self.len()
                (start_idx+multiple..self.len()).step_by(multiple).for_each(|idx| self[idx] = None);
            }
        }

        delegate! {
            to self.seive {
                pub fn len(&self) -> usize;
            }
        }

        fn remove_all_multiple_of_current_prime(&mut self) {
            self.checked_remove_multiple(self[self.current_checked_prime_idx].unwrap(),self.current_checked_prime_idx)
        }

    }

    // 1. Correct the upper bound calculation for the loop
    let limit = ((range as f64).sqrt() as usize / seive.spacing );
    for number_to_check in (0..limit) {
        if let Some(prime) = seive[number_to_check] {
            seive.current_checked_prime_idx=number_to_check;
            seive.remove_all_multiple_of_current_prime();
        }
    }
    println!("{:?}", seive)
}

//12.5Gb, 2 billion, 14 minutes 40 seconds