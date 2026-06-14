use crate::seg::SegSeive;
use crate::primes::MISPrimes;
use crate::smallest_uint;
use num_traits::{PrimInt, Unsigned};

pub struct FullSeive<I,const SEG_SIZE:usize> 
where
    I: PrimInt + Unsigned + std::ops::AddAssign, // This restricts I to unsigned integers only!
{
    pub seive: SegSeive<SEG_SIZE>,
    pub primes:MISPrimes<I,SEG_SIZE,2,3>
}

#[macro_export]
macro_rules! new_full_seive {
    ($seg_size:expr) => {
        $crate::FullSeive::<$crate::smallest_uint!($seg_size), $seg_size> {
            seive: $crate::SegSeive::<$seg_size>::new(),
            primes: $crate::MISPrimes::<$crate::smallest_uint!($seg_size), $seg_size, 2, 3>::new() 
        }
    };
}

impl <I,const SEG_SIZE:usize> FullSeive<I,SEG_SIZE> 
where
    I: PrimInt + Unsigned + std::ops::AddAssign, // This restricts I to unsigned integers only!
{
    
//     pub fn new(range:usize) -> Self {
//         let mut result = Self {
//             seive: SegSeive::<SEG_SIZE>::new(range),
//             primes: MISPrimes::new(range)
//         };
//         result.primes.primes.push(3);
//         result
//     }

//     pub fn filter_primes(&mut self) {
//         self.seive.remove_multiples_in_iter(self.primes.iter_of_primes_to_check(self.seive.range));
//         while let Some(prime_idx) = self.seive.find_set(self.seive.last_prime_idx.unwrap_or(0)) {
//             let prime_val = self.seive.guess_dex(prime_idx);
//             self.primes.primes.push(prime_val);
//             self.seive.seive[prime_idx] = false;
//             if prime_val<=Primes::max_factor_to_check(self.seive.range) {
//                 self.seive.remove_multiple(prime_val);
//             }
//             self.seive.last_prime_idx = Some(prime_idx);
//         }
//     }

//     pub fn filter_range(&mut self) {
//         for _ in 0..self.seive.num_of_seive_bumps_floor() {
//             self.filter_primes();
//             self.seive.bump_seive(); 
//         }
//         let range_local_idx = self.seive.local_ranges_idx().unwrap_or(0);
//         self.seive.seive[range_local_idx..SEG_SIZE].fill(false);
//         self.filter_primes();
//     }
} 

/*
Compiled: 1 billion: 390MB ram used, ./seg_seive_bool 1000000000  4.38s user 1.48s system 70% cpu 8.285 total
Compiled: 10 billion: 3363MB ram used, ./seg_seive_bool 10000000000  48.68s user 16.39s system 76% cpu 1:24.93 total

Uncompiled 1 billion:390MB ram used, cargo run --bin seg_seive_bool -- 1000000000  27.67s user 1.65s system 93% cpu 31.524 total

*/